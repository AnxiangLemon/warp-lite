param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $WarpArgs
)

$ErrorActionPreference = 'Stop'
$env:RUSTUP_TOOLCHAIN = if ($env:RUSTUP_TOOLCHAIN) { $env:RUSTUP_TOOLCHAIN } else { '1.92.0-x86_64-pc-windows-msvc' }

& cargo run --bin warp-lite --no-default-features --features 'local_tty,warp_lite' -- @WarpArgs
exit $LASTEXITCODE
