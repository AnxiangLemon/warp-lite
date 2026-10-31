"""Verify generator framing and lifecycle against real PowerShell runtimes."""

import argparse
from pathlib import Path
import re
import subprocess


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--shell", action="append", required=True)
args = parser.parse_args()
fixture = Path(__file__).with_suffix(".ps1")
expected = {"first;alpha;0", "second;beta;0", "failure;;1", "cancel;;1"}
for shell in args.shell:
    result = subprocess.run(
        [shell, "-NoLogo", "-NoProfile", "-File", str(fixture)],
        capture_output=True,
        timeout=45,
    )
    output = result.stdout.decode("utf-8", errors="replace")
    frames = re.findall(r"\x1b\]9277;A\x07(\d+);([0-9A-F]+)\x1b\]9277;B\x07", output)
    payloads = {bytes.fromhex(value).decode("utf-8") for _, value in frames}
    assert result.returncode == 0, result.stderr.decode("utf-8", errors="replace")
    assert "bootstrap-generator-tests-passed" in output, output
    assert len(frames) == len(expected) and payloads == expected, payloads
    assert all(int(length) == len(value) for length, value in frames), frames
    print(f"PASS: {shell}: batch output, failure, cancellation, pool growth and cleanup")
