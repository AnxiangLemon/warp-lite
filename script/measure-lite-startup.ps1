param(
    [Parameter(Mandatory)] [string] $BeforeExecutable,
    [Parameter(Mandatory)] [string] $AfterExecutable,
    [Parameter(Mandatory)] [string] $SeedDirectory,
    [Parameter(Mandatory)] [string] $OutputPath,
    [int] $Pairs = 5,
    [int] $ExpectedShells = 5,
    [int] $ExpectedPowerShellPrompts = 0
)

$ErrorActionPreference = 'Stop'
$executables = @{
    before = (Resolve-Path -LiteralPath $BeforeExecutable).Path
    after = (Resolve-Path -LiteralPath $AfterExecutable).Path
}
$seed = (Resolve-Path -LiteralPath $SeedDirectory).Path
$runId = [Guid]::NewGuid().ToString('N').Substring(0, 12)
$reports = @()
$originalProfile = $env:WARP_DATA_PROFILE
$originalLog = $env:RUST_LOG
$originalPromptProbe = $env:WARP_STARTUP_PROMPT_PROBE_DIR
function Get-PackageInputs([string] $Executable) {
    $directory = Split-Path $Executable
    @('warp-lite.exe', 'pwsh.ps1', 'conpty.dll', 'backgrounds/background.png', 'backgrounds/background.jpg') | ForEach-Object {
        $path = Join-Path $directory $_
        if (Test-Path -LiteralPath $path -PathType Leaf) {
            [ordered]@{name=$_; bytes=(Get-Item -LiteralPath $path).Length; sha256=(Get-FileHash -LiteralPath $path).Hash}
        }
    }
}
try {
    for ($round = 0; $round -le $Pairs; $round++) {
        $variants = if ($round % 2) { @('after', 'before') } else { @('before', 'after') }
        foreach ($variant in $variants) {
            $executable = $executables[$variant]
            $profile = "Startup-$runId-$round-$variant"
            $profileRoot = Join-Path $env:LOCALAPPDATA "warp/WarpLite-$profile"
            if (Test-Path -LiteralPath $profileRoot) { throw "Profile already exists: $profileRoot" }
            [void][IO.Directory]::CreateDirectory((Join-Path $profileRoot 'data'))
            [void][IO.Directory]::CreateDirectory((Join-Path $profileRoot 'config'))
            Copy-Item -LiteralPath (Join-Path $seed 'warp.sqlite') -Destination (Join-Path $profileRoot 'data/warp.sqlite')
            Copy-Item -LiteralPath (Join-Path $seed 'settings.toml') -Destination (Join-Path $profileRoot 'config/settings.toml')
            $env:WARP_DATA_PROFILE = $profile
            $env:RUST_LOG = 'info,warp::startup=debug,warpui::startup=debug'
            $promptDirectory = Join-Path $profileRoot 'data/prompt-readiness'
            if ($ExpectedPowerShellPrompts -gt 0) {
                [void][IO.Directory]::CreateDirectory($promptDirectory)
                $env:WARP_STARTUP_PROMPT_PROBE_DIR = $promptDirectory
            }
            $logPath = Join-Path $profileRoot 'data/logs/warp-lite.log'
            $startedTicks = [Diagnostics.Stopwatch]::GetTimestamp()
            $timer = [Diagnostics.Stopwatch]::StartNew()
            $frameObservedMs = $null
            $shellsObservedMs = $null
            $probe = Start-Process -FilePath $executable -WorkingDirectory (Split-Path $executable) -WindowStyle Hidden -PassThru
            try {
                do {
                    Start-Sleep -Milliseconds 20
                    $probe.Refresh()
                    if ($probe.HasExited) { throw 'The isolated probe exited unexpectedly.' }
                    if ($timer.Elapsed.TotalSeconds -gt 45) { throw 'Timed out waiting for first frame and restored shells.' }
                    $lines = if (Test-Path -LiteralPath $logPath) { @(Get-Content -LiteralPath $logPath) } else { @() }
                    $frames = @($lines | Where-Object { $_ -match 'first_frame elapsed_ms=' })
                    if ($frames.Count -gt 0 -and $null -eq $frameObservedMs) {
                        $frameObservedMs = [Math]::Round($timer.Elapsed.TotalMilliseconds, 3)
                    }
                    $shells = @($lines | Where-Object { $_ -match 'shell_ready session_id=' })
                    if ($shells.Count -ge $ExpectedShells -and $null -eq $shellsObservedMs) {
                        $shellsObservedMs = [Math]::Round($timer.Elapsed.TotalMilliseconds, 3)
                    }
                    $prompts = @()
                    if ($ExpectedPowerShellPrompts -gt 0) {
                        $prompts = @(Get-ChildItem -LiteralPath $promptDirectory -Filter '*.ready' | ForEach-Object {
                            $ticks = Get-Content -LiteralPath $_.FullName -Raw
                            if ($ticks -match '^\d+$') {
                                [ordered]@{session_id=$_.BaseName; process_to_prompt_ms=[Math]::Round(([double]$ticks - $startedTicks) * 1000 / [Diagnostics.Stopwatch]::Frequency, 3)}
                            }
                        })
                    }
                } until ($frames.Count -gt 0 -and $shells.Count -ge $ExpectedShells -and $prompts.Count -ge $ExpectedPowerShellPrompts)
                $frameMs = [double]([regex]::Match($frames[0], 'elapsed_ms=([0-9.]+)').Groups[1].Value)
                $shellDetails = @($shells | ForEach-Object {
                    $match = [regex]::Match($_, 'session_id=(\S+) elapsed_ms=([0-9.]+) bootstrap_seconds=(\S+) rcfiles_seconds=(\S+)')
                    [ordered]@{session_id=$match.Groups[1].Value; elapsed_ms=[double]$match.Groups[2].Value; bootstrap_seconds=$match.Groups[3].Value; rcfiles_seconds=$match.Groups[4].Value}
                })
                $stages = @($lines | Where-Object { $_ -match 'Lite startup timings:' })
                $report = [ordered]@{
                    round=$round; warmup=($round -eq 0); variant=$variant; process_id=$probe.Id
                    first_frame_ms=$frameMs
                    process_to_first_frame_observed_ms=$frameObservedMs
                    first_shell_ready_ms=($shellDetails.elapsed_ms | Measure-Object -Minimum).Minimum
                    all_shells_ready_ms=($shellDetails.elapsed_ms | Measure-Object -Maximum).Maximum
                    process_to_all_shells_observed_ms=$shellsObservedMs
                    process_to_all_powershell_prompts_ms=($prompts.process_to_prompt_ms | Measure-Object -Maximum).Maximum
                    powershell_prompts=$prompts
                    shell_count=$shellDetails.Count; shells=$shellDetails
                    stages=(($stages[0] -replace '^.*Lite startup timings: ', '') | ConvertFrom-Json)
                    diagnostics=@($lines | Where-Object { $_ -match '\b(first_frame|cwd_resolved|shell_resolved|pty_start|pty_ready|shell_ready|gpu_surface|gpu_device|rect_pipeline|glyph_pipeline|image_pipeline) ' })
                    profile=$profileRoot
                }
                $reports += $report
                Write-Output "$variant round=$round first_frame_ms=$frameMs all_shells_ready_ms=$($report.all_shells_ready_ms)"
                Start-Sleep -Milliseconds 500
            } finally {
                $probe.Refresh()
                if (!$probe.HasExited) {
                    if ((Get-Process -Id $probe.Id).Path -ne $executable) { throw 'Probe identity mismatch.' }
                    $children = @(Get-CimInstance Win32_Process -Filter "ParentProcessId=$($probe.Id)")
                    # Stop our recovery helper first, so cleanup cannot relaunch the probe.
                    foreach ($helper in ($children | Where-Object { $_.ExecutablePath -eq $executable -and $_.CommandLine -match '--parent-handle' })) {
                        $helperProcess = Get-Process -Id $helper.ProcessId -ErrorAction SilentlyContinue
                        if ($helperProcess) {
                            Stop-Process -Id $helper.ProcessId -ErrorAction SilentlyContinue
                            [void]$helperProcess.WaitForExit(2000)
                        }
                    }
                    Stop-Process -Id $probe.Id
                    [void]$probe.WaitForExit(3000)
                    $survivors = @(Get-CimInstance Win32_Process -Filter "ParentProcessId=$($probe.Id)")
                    foreach ($current in $survivors) {
                        $child = $children | Where-Object ProcessId -eq $current.ProcessId
                        if ($child -and $current.CreationDate -eq $child.CreationDate) {
                            Stop-Process -Id $current.ProcessId -ErrorAction SilentlyContinue
                        }
                    }
                }
            }
        }
    }
} finally {
    $env:WARP_DATA_PROFILE = $originalProfile
    $env:RUST_LOG = $originalLog
    $env:WARP_STARTUP_PROMPT_PROBE_DIR = $originalPromptProbe
    [ordered]@{
        scope='Optimized Lite builds, identical session snapshot/settings/background/runtime DLLs; one warmup pair, alternating order; no WSL shutdown or cache clearing'
        before_sha256=(Get-FileHash -LiteralPath $executables.before).Hash
        after_sha256=(Get-FileHash -LiteralPath $executables.after).Hash
        before_files=@(Get-PackageInputs $executables.before)
        after_files=@(Get-PackageInputs $executables.after)
        seed_sha256=(Get-FileHash -LiteralPath (Join-Path $seed 'warp.sqlite')).Hash
        samples=$reports
    } | ConvertTo-Json -Depth 12 | Set-Content -LiteralPath $OutputPath -Encoding utf8
}
