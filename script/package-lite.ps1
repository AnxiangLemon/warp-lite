param(
    [switch] $Debug,
    [switch] $SkipBuild,
    [switch] $SystemShaderCompiler,
    [string] $OutputDirectory,
    [string] $ExecutablePath
)

$ErrorActionPreference = 'Stop'
if ($ExecutablePath -and !$SkipBuild) {
    throw 'ExecutablePath is only supported with SkipBuild.'
}
if ($ExecutablePath) {
    $ExecutablePath = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($ExecutablePath)
}
$repoRoot = Split-Path -Parent $PSScriptRoot
if (!$OutputDirectory) {
    $OutputDirectory = Join-Path $repoRoot 'dist\Warp Lite'
}
$packageRoot = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($OutputDirectory)
$packageRoot = [System.IO.Path]::GetFullPath($packageRoot).TrimEnd('\', '/')
if (!$packageRoot -or $packageRoot -eq [System.IO.Path]::GetPathRoot($packageRoot).TrimEnd('\', '/')) {
    throw 'The package output must be a dedicated directory, not a filesystem root.'
}

function Assert-LitePackageDestination([string] $Path) {
    $resolvedPath = [System.IO.Path]::GetFullPath($Path)
    if (!$resolvedPath.StartsWith($packageRoot + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Package destination is outside the output directory: $resolvedPath"
    }
    if (Test-Path -LiteralPath $resolvedPath -PathType Container) {
        throw "A package file destination is an existing directory: $resolvedPath"
    }
    $currentPath = $resolvedPath
    while ($currentPath) {
        $item = Get-Item -LiteralPath $currentPath -Force -ErrorAction SilentlyContinue
        if ($item) {
            if ($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) {
                throw "Package destinations must not pass through a symlink or junction: $currentPath"
            }
        }
        $currentPath = Split-Path -Parent $currentPath
    }
    return $resolvedPath
}

function Resolve-LitePackageSource([string[]] $Candidates) {
    foreach ($candidate in $Candidates) {
        if (Test-Path -LiteralPath $candidate -PathType Leaf) {
            return [System.IO.Path]::GetFullPath($candidate)
        }
    }
    throw "Required package file not found. Checked: $($Candidates -join ', ')"
}

if ($SystemShaderCompiler) {
    foreach ($dll in @('dxcompiler.dll', 'dxil.dll')) {
        $existingCompiler = Assert-LitePackageDestination (Join-Path $packageRoot $dll)
        if (Test-Path -LiteralPath $existingCompiler -PathType Leaf) {
            throw "SystemShaderCompiler requires an output directory without bundled shader DLLs. Use a new OutputDirectory; existing file: $existingCompiler"
        }
    }
}

Push-Location -LiteralPath $repoRoot
try {
    $metadataJson = & cargo metadata --locked --format-version 1 --no-deps
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed with exit code $LASTEXITCODE"
    }
    $metadata = ($metadataJson -join [Environment]::NewLine) | ConvertFrom-Json
    if (!$metadata.target_directory) {
        throw 'cargo metadata did not return target_directory.'
    }

    $profile = if ($Debug) { 'debug' } else { 'lite' }
    $targetRoot = [System.IO.Path]::GetFullPath($metadata.target_directory)
    $targetTriple = $env:CARGO_BUILD_TARGET
    $architecture = switch ([System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture.ToString()) {
        'X64' { 'x64' }
        'Arm64' { 'arm64' }
        default { throw 'Warp Lite packaging supports Windows x64 and ARM64.' }
    }
    if ($targetTriple) {
        $architecture = switch ($targetTriple) {
            'x86_64-pc-windows-msvc' { 'x64' }
            'aarch64-pc-windows-msvc' { 'arm64' }
            default { throw "Unsupported CARGO_BUILD_TARGET: $targetTriple" }
        }
        $targetRoot = Join-Path $targetRoot $targetTriple
    }
    $buildDirectory = Join-Path $targetRoot $profile

    if (!$SkipBuild) {
        $buildArgs = @('build', '--locked', '-p', 'warp', '--bin', 'warp-lite', '--no-default-features', '--features', 'warp_lite')
        if (!$Debug) {
            $buildArgs += @('--profile', 'lite')
        }
        & cargo @buildArgs --message-format=json-render-diagnostics | ForEach-Object {
            $buildMessage = $_ | ConvertFrom-Json -ErrorAction SilentlyContinue
            if ($buildMessage.reason -eq 'compiler-artifact' -and $buildMessage.target.name -eq 'warp-lite' -and $buildMessage.executable) {
                $ExecutablePath = $buildMessage.executable
            }
            if ($buildMessage.reason -eq 'compiler-message' -and $buildMessage.message.rendered) {
                Write-Host $buildMessage.message.rendered
            }
        }
        if ($LASTEXITCODE -ne 0) {
            throw "cargo build failed with exit code $LASTEXITCODE"
        }
        if (!$ExecutablePath) {
            throw 'Cargo did not report a warp-lite executable artifact.'
        }
    }
    if ($ExecutablePath) {
        $ExecutablePath = Resolve-LitePackageSource -Candidates @($ExecutablePath)
        $buildDirectory = Split-Path -Parent $ExecutablePath
        $artifactTarget = Split-Path -Leaf (Split-Path -Parent $buildDirectory)
        if ($artifactTarget -eq 'x86_64-pc-windows-msvc') { $architecture = 'x64' }
        if ($artifactTarget -eq 'aarch64-pc-windows-msvc') { $architecture = 'arm64' }
    } else {
        $ExecutablePath = Resolve-LitePackageSource -Candidates @((Join-Path $buildDirectory 'warp-lite.exe'))
    }

    $windowsAssets = Join-Path $repoRoot "app\assets\windows\$architecture"
    $files = @(
        [pscustomobject]@{
            Source = $ExecutablePath
            RelativePath = 'warp-lite.exe'
        }
    )
    $runtimeDlls = @('conpty.dll', 'vcruntime140.dll', 'vcruntime140_1.dll', 'msvcp140.dll')
    if (!$SystemShaderCompiler) {
        $runtimeDlls += @('dxcompiler.dll', 'dxil.dll')
    }
    foreach ($dll in $runtimeDlls) {
        $files += [pscustomobject]@{
            Source = Resolve-LitePackageSource -Candidates @((Join-Path $buildDirectory $dll), (Join-Path $windowsAssets $dll))
            RelativePath = $dll
        }
    }
    $files += @(
        [pscustomobject]@{
            Source = Resolve-LitePackageSource -Candidates @((Join-Path $buildDirectory "$architecture\OpenConsole.exe"), (Join-Path $windowsAssets 'OpenConsole.exe'))
            RelativePath = "$architecture\OpenConsole.exe"
        }
        [pscustomobject]@{
            Source = Join-Path $repoRoot 'app\assets\bundled\bootstrap\pwsh.ps1'
            RelativePath = 'pwsh.ps1'
        }
        [pscustomobject]@{ Source = Join-Path $repoRoot 'LICENSE-AGPL'; RelativePath = 'LICENSE-AGPL' }
        [pscustomobject]@{ Source = Join-Path $repoRoot 'LICENSE-MIT'; RelativePath = 'LICENSE-MIT' }
    )

    $additionalLicenses = @(
        @{ Source = 'app\assets\windows\LICENSE-WINDOWS-TERMINAL'; Name = 'WINDOWS-TERMINAL.txt' }
        @{ Source = 'app\assets\windows\LICENSE-DXC'; Name = 'DIRECTX-SHADER-COMPILER.txt' }
        @{ Source = 'crates\warp_terminal\src\model\LICENSE-ALACRITTY'; Name = 'ALACRITTY.txt' }
        @{ Source = 'app\assets\bundled\fonts\hack\LICENSE.md'; Name = 'HACK-FONT.md' }
        @{ Source = 'app\assets\bundled\fonts\roboto\LICENSE.txt'; Name = 'ROBOTO-FONT.txt' }
        @{ Source = 'app\assets\bundled\bootstrap\bash-preexec-LICENSE.md'; Name = 'BASH-PREEXEC.md' }
        @{ Source = 'app\src\server\telemetry\LICENSE-RUDDER-SDK-RUST.txt'; Name = 'RUDDER-SDK-RUST.txt' }
        @{ Source = 'app\src\code_review\GITHUB-DESKTOP-LICENSE'; Name = 'GITHUB-DESKTOP.txt' }
    )
    foreach ($license in $additionalLicenses) {
        $files += [pscustomobject]@{
            Source = Join-Path $repoRoot $license.Source
            RelativePath = Join-Path 'licenses' $license.Name
        }
    }

    $thirdPartyLicenses = @(
        (Join-Path $buildDirectory 'resources\THIRD_PARTY_LICENSES.txt')
        (Join-Path $buildDirectory 'THIRD_PARTY_LICENSES.txt')
        (Join-Path $repoRoot 'resources\THIRD_PARTY_LICENSES.txt')
        (Join-Path $repoRoot 'THIRD_PARTY_LICENSES.txt')
    ) | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -First 1
    if ($thirdPartyLicenses) {
        $files += [pscustomobject]@{
            Source = $thirdPartyLicenses
            RelativePath = 'resources\THIRD_PARTY_LICENSES.txt'
        }
    }

    foreach ($file in $files) {
        $file.Source = Resolve-LitePackageSource -Candidates @($file.Source)
        $destination = Assert-LitePackageDestination (Join-Path $packageRoot $file.RelativePath)
        if ($destination.Equals($file.Source, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "Package input and output must be different files: $destination"
        }
    }
    $packageInfoPath = Assert-LitePackageDestination (Join-Path $packageRoot 'PACKAGE-INFO.json')
    $backgroundReadmePath = Assert-LitePackageDestination (Join-Path $packageRoot 'backgrounds\使用说明.txt')

    foreach ($file in $files) {
        $destination = Assert-LitePackageDestination (Join-Path $packageRoot $file.RelativePath)
        [void][System.IO.Directory]::CreateDirectory((Split-Path -Parent $destination))
        Copy-Item -LiteralPath $file.Source -Destination $destination -Force
    }

    [void][System.IO.Directory]::CreateDirectory((Split-Path -Parent $backgroundReadmePath))
    $backgroundReadme = @'
将 PNG 图片命名为 background.png 放在此目录，重启程序后生效。
没有图片或图片损坏时使用纯色主题。
仅支持大小不超过 8 MiB、边长不超过 4096 像素的静态 PNG。
'@
    [System.IO.File]::WriteAllText($backgroundReadmePath, $backgroundReadme, [System.Text.UTF8Encoding]::new($false))

    $binaryPath = Join-Path $packageRoot 'warp-lite.exe'
    $packageInfo = [ordered]@{
        application = 'Warp Lite'
        profile = $profile
        architecture = $architecture
        shader_compiler = if ($SystemShaderCompiler) { 'system_fxc' } else { 'dynamic_dxc' }
        background_directory = 'backgrounds'
        fixed_background_file = 'backgrounds/background.png'
        background_load_policy = 'startup_only_with_solid_fallback'
        background_limits = @{ file_bytes = 8388608; max_dimension = 4096; rendered_max_dimension = 2048 }
        external_background_files = @()
        features = @('warp_lite')
        build_invoked = !$SkipBuild
        source_directory = $repoRoot
        target_directory = $metadata.target_directory
        packaged_at_utc = [DateTime]::UtcNow.ToString('o')
        executable_sha256 = (Get-FileHash -LiteralPath $binaryPath -Algorithm SHA256).Hash
        third_party_licenses_included = [bool]$thirdPartyLicenses
        third_party_license_generator = 'script/windows/prepare_bundled_resources.ps1'
        files = @($files | ForEach-Object { $_.RelativePath }) + @('backgrounds\使用说明.txt')
    }
    $packageInfo | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $packageInfoPath -Encoding utf8

    if (!$thirdPartyLicenses) {
        Write-Warning 'THIRD_PARTY_LICENSES.txt was not found. Included individual license files are not a complete Cargo dependency attribution list. The upstream generator is script/windows/prepare_bundled_resources.ps1; generate the list and package again before distribution.'
    }
    [pscustomobject]@{
        Directory = $packageRoot
        Executable = $binaryPath
        Profile = $profile
        Architecture = $architecture
        ShaderCompiler = $packageInfo.shader_compiler
        ThirdPartyLicensesIncluded = [bool]$thirdPartyLicenses
    }
} finally {
    Pop-Location
}
