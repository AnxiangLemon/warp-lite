param(
    [switch] $Release,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $WarpArgs
)

$ErrorActionPreference = 'Stop'

# Debug builds are ~350MB+ because they keep full debuginfo and skip LTO.
# Prefer -Release for a distributable binary (still large: full Warp GUI stack).
$profileArgs = @()
if ($Release) {
    $profileArgs = @('--release')
}

& cargo run @profileArgs --bin warp-lite --no-default-features --features 'warp_lite' -- @WarpArgs
exit $LASTEXITCODE
