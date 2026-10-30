param(
    [string] $Target = 'x86_64-pc-windows-msvc',
    [string] $ReportPath
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $repoRoot
try {
    $tree = @(& cargo tree --locked --offline -p warp --no-default-features --features warp_lite --target $Target --edges normal,no-proc-macro --prefix none --format '{p}')
    if ($LASTEXITCODE -ne 0) {
        throw 'Could not inspect the Lite dependency graph.'
    }
    $packages = @($tree | ForEach-Object {
        if ($_ -match '^([\w-]+) v([^ (]+)') {
            [pscustomobject]@{ Name = $Matches[1]; Version = $Matches[2] }
        }
    } | Sort-Object Name, Version -Unique)
    $removedServices = '^(aws-(config|credential-types|runtime|sdk-.+|sigv4|smithy-.+|types)|tantivy(-.+)?|tink-.+|hpke|git2|libgit2-sys|rquickjs(-.+)?|warp_js|opentelemetry-otlp|mermaid_to_svg|syntect|html5ever|markup5ever|ipynb)$'
    $parserSupport = @('arborium-highlight', 'arborium-theme', 'arborium-tree-sitter')
    $unexpected = @($packages | Where-Object {
        $_.Name -match $removedServices -or (
            $_.Name.StartsWith('arborium-') -and $_.Name -notin $parserSupport
        )
    })
    $report = [ordered]@{
        target = $Target
        runtime_package_count = $packages.Count
        unexpected_packages = $unexpected
        verified_at_utc = [DateTime]::UtcNow.ToString('o')
    }
    if ($ReportPath) {
        $report | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $ReportPath -Encoding utf8
    }
    if ($unexpected.Count -gt 0) {
        throw "Lite includes removed service dependencies: $(($unexpected.Name | Sort-Object -Unique) -join ', ')"
    }
    [pscustomobject]$report
} finally {
    Pop-Location
}
