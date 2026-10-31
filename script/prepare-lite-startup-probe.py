"""Copy a Lite package and add a one-shot PowerShell prompt readiness probe."""

import argparse
from pathlib import Path
import shutil


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("source", type=Path)
parser.add_argument("destination", type=Path)
args = parser.parse_args()
source = args.source.resolve()
destination = args.destination.resolve()
if destination == source or source in destination.parents or destination.exists():
    parser.error("Destination must be a new directory outside the source package.")
script = (source / "pwsh.ps1").read_text(encoding="utf-8")
needle = """            Warp-Send-JsonMessage $precmdMsg
        }
    }
"""
marker = """        if ($global:WARP_BOOTSTRAPPED -eq 1 -and -not $global:_warpStartupPromptRecorded) {
            $ticks = [Diagnostics.Stopwatch]::GetTimestamp().ToString([Globalization.CultureInfo]::InvariantCulture)
            [IO.File]::WriteAllText([IO.Path]::Combine($env:WARP_STARTUP_PROMPT_PROBE_DIR, "$global:_warpSessionId.ready"), $ticks)
            $global:_warpStartupPromptRecorded = $true
        }
"""
if script.count(needle) != 1 or "_warpStartupPromptRecorded" in script:
    parser.error("Unsupported or already instrumented PowerShell bootstrap.")
shutil.copytree(source, destination)
script = script.replace(needle, needle.removesuffix("    }\n") + marker + "    }\n", 1)
(destination / "pwsh.ps1").write_text(script, encoding="utf-8")
print(f"Prepared {destination}; measure with -ExpectedPowerShellPrompts > 0.")
