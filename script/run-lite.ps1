param(
    [switch] $Release,
    [switch] $BuildOnly,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $WarpArgs
)

$ErrorActionPreference = 'Stop'

$cargoArgs = @('run')
if ($BuildOnly) {
    $cargoArgs = @('build')
}
if ($Release) {
    $cargoArgs += @('--profile', 'lite')
}
$cargoArgs += @('--bin', 'warp-lite', '--no-default-features', '--features', 'warp_lite', '--locked')
if (!$BuildOnly) {
    $cargoArgs += '--'
    $cargoArgs += $WarpArgs
}

Push-Location -LiteralPath (Split-Path -Parent $PSScriptRoot)
try {
    & cargo @cargoArgs
    $cargoExitCode = $LASTEXITCODE
} finally {
    Pop-Location
}
exit $cargoExitCode
