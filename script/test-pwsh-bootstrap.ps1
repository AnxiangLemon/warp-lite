$ErrorActionPreference = 'Stop'
$sourcePath = Join-Path $PSScriptRoot '../app/assets/bundled/bootstrap/pwsh.ps1'
$tokens = $null
$parseErrors = $null
$ast = [System.Management.Automation.Language.Parser]::ParseFile($sourcePath, [ref]$tokens, [ref]$parseErrors)
if ($parseErrors.Count) { throw ($parseErrors | Out-String) }

# Exercise the real generator functions without sourcing profiles or changing the parent shell.
$names = @('Warp-Initialize-GeneratorPools', 'Warp-Run-GeneratorCommandImpl', 'Warp-Stop-ActiveThread', 'Warp-Clean-CompletedThread')
$functions = $ast.FindAll({param($node) $node -is [System.Management.Automation.Language.FunctionDefinitionAst] -and $node.Name -in $names}, $true)
if ($functions.Count -ne $names.Count) { throw 'Missing bootstrap generator functions.' }
$common = $ast.Find({param($node) $node -is [System.Management.Automation.Language.AssignmentStatementAst] -and $node.Left.Extent.Text -eq '$warpCommon'}, $true)
$commandType = $ast.Find({param($node) $node -is [System.Management.Automation.Language.TypeDefinitionAst] -and $node.Name -eq 'WarpGeneratorCommand'}, $true)
$fixture = @'
$script:innerRunspacePool = $null
$script:outerRunspacePool = $null
$script:inBandCommandCount = 0
$script:threadInner = @{}
$script:threadOuter = @{}
function Wait-GeneratorJobs {
    $timer = [Diagnostics.Stopwatch]::StartNew()
    while (@($script:threadOuter.Values | Where-Object { [int]$_.InvocationStateInfo.State -le 2 }).Count) {
        if ($timer.Elapsed.TotalSeconds -gt 15) { throw 'Generator jobs timed out.' }
        Start-Sleep -Milliseconds 10
    }
    foreach ($job in $script:threadOuter.Values) {
        if ($job.InvocationStateInfo.State -ne 'Completed') { throw $job.InvocationStateInfo.Reason }
    }
}
function Invoke-BootstrapTest {
    if ($null -ne $script:innerRunspacePool -or $null -ne $script:outerRunspacePool) { throw 'Pools must start unloaded.' }
    try {
        Warp-Run-GeneratorCommandImpl @(
            [WarpGeneratorCommand]@{CommandId='first'; Command="Write-Output 'alpha'"},
            [WarpGeneratorCommand]@{CommandId='second'; Command="Write-Output 'beta'"},
            [WarpGeneratorCommand]@{CommandId='failure'; Command="throw 'expected test failure'"}
        )
        if ($script:innerRunspacePool.GetMinRunspaces() -ne 1 -or $script:outerRunspacePool.GetMinRunspaces() -ne 1) { throw 'Pools must grow from one runspace.' }
        if ($script:innerRunspacePool.GetMaxRunspaces() -ne 20 -or $script:outerRunspacePool.GetMaxRunspaces() -ne 10) { throw 'Concurrency limits changed.' }
        if (@($script:threadInner.Values | Where-Object {$null -eq $_}).Count) { throw 'Every inner job needs a tracked PowerShell instance.' }
        Wait-GeneratorJobs
        Warp-Clean-CompletedThread
        if ($script:threadInner.Count -or $script:threadOuter.Count) {
            $states = @($script:threadInner.Values | ForEach-Object { $_.InvocationStateInfo.State }) -join ','
            throw "Completed jobs were not released: inner=$($script:threadInner.Count), outer=$($script:threadOuter.Count), states=$states"
        }

        $firstPool = $script:innerRunspacePool
        Warp-Run-GeneratorCommandImpl @([WarpGeneratorCommand]@{CommandId='cancel'; Command='Start-Sleep -Seconds 10'})
        if (![object]::ReferenceEquals($firstPool, $script:innerRunspacePool)) { throw 'Generator pools were not reused.' }
        $timer = [Diagnostics.Stopwatch]::StartNew()
        while (@($script:threadInner.Values | Where-Object {$_.InvocationStateInfo.State -eq 'Running'}).Count -eq 0) {
            if ($timer.Elapsed.TotalSeconds -gt 10) { throw 'Cancellation target did not start.' }
            Start-Sleep -Milliseconds 10
        }
        Warp-Stop-ActiveThread
        Wait-GeneratorJobs
        Warp-Clean-CompletedThread
        if ($script:threadInner.Count -or $script:threadOuter.Count) { throw 'Canceled jobs were not released.' }
        Write-Output 'bootstrap-generator-tests-passed'
    } finally {
        if ($null -ne $script:innerRunspacePool) { $script:innerRunspacePool.Dispose() }
        if ($null -ne $script:outerRunspacePool) { $script:outerRunspacePool.Dispose() }
    }
}
Export-ModuleMember -Function Invoke-BootstrapTest
'@
$moduleSource = $commandType.Extent.Text + "`n" + $common.Extent.Text + "`n" + ($functions.Extent.Text -join "`n") + "`n" + $fixture
$module = New-Module -Name WarpBootstrapTest -ScriptBlock ([scriptblock]::Create($moduleSource))
Import-Module $module
Invoke-BootstrapTest
