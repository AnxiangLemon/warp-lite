param(
    [Parameter(Mandatory)]
    [int] $ProcessId,
    [ValidateRange(1, 20)]
    [int] $Samples = 5,
    [ValidateRange(1, 10)]
    [int] $IntervalSeconds = 2,
    [string] $ReportPath
)

$ErrorActionPreference = 'Stop'
$process = Get-Process -Id $ProcessId
$executable = $process.Path
if ([System.IO.Path]::GetFileName($executable) -ne 'warp-lite.exe') {
    throw 'ProcessId must identify the Warp Lite process being measured.'
}
$startedAt = $process.StartTime.ToUniversalTime().ToString('o')
$measurements = @(for ($sample = 0; $sample -lt $Samples; $sample++) {
    $current = Get-Process -Id $ProcessId
    if ($current.StartTime.ToUniversalTime().ToString('o') -ne $startedAt) {
        throw 'The measured process exited and its ID was reused.'
    }
    $memory = Get-CimInstance Win32_PerfFormattedData_PerfProc_Process -Filter "IDProcess=$ProcessId"
    if (!$memory) { throw 'No memory counters were returned for the process.' }
    [pscustomobject]@{
        timestamp_utc = [DateTime]::UtcNow.ToString('o')
        private_working_set_mib = [Math]::Round($memory.WorkingSetPrivate / 1MB, 2)
        working_set_mib = [Math]::Round($memory.WorkingSet / 1MB, 2)
        private_committed_mib = [Math]::Round($memory.PrivateBytes / 1MB, 2)
        threads = $memory.ThreadCount
    }
    if ($sample + 1 -lt $Samples) { Start-Sleep -Seconds $IntervalSeconds }
})
$report = [ordered]@{
    executable = $executable
    executable_sha256 = (Get-FileHash -LiteralPath $executable -Algorithm SHA256).Hash
    executable_mib = [Math]::Round((Get-Item -LiteralPath $executable).Length / 1MB, 2)
    process_id = $ProcessId
    process_started_at_utc = $startedAt
    window_title = (Get-Process -Id $ProcessId).MainWindowTitle
    scope = 'main_process_only; shell and crash-recovery helper excluded'
    samples = $measurements
}
$json = $report | ConvertTo-Json -Depth 4
if ($ReportPath) { $json | Set-Content -LiteralPath $ReportPath -Encoding utf8 }
$json
