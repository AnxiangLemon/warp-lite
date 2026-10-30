#!/usr/bin/env python3
"""Verify local dependency provenance and reject upstream Git resolution."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ledger-only", action="store_true", help="verify without invoking Cargo")
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    vendor = root / "vendor" / "upstream"
    ledger = json.loads((root / "vendor" / "sources.json").read_text(encoding="utf-8"))
    expected_packages = set()
    for package in ledger["packages"]:
        directory = (vendor / f"{package['name']}-{package['version']}").resolve()
        if not directory.is_relative_to(vendor.resolve()):
            raise SystemExit(f"Invalid vendor path: {directory}")
        expected_packages.add(directory.name)
        for file in directory.rglob("*"):
            if file.is_file() and file.stat().st_size < 1024:
                if file.read_bytes().startswith(b"version https://git-lfs.github.com/spec/v1"):
                    raise SystemExit(f"Unresolved LFS pointer in local dependency: {file}")
        actual = {
            file.relative_to(directory).as_posix(): hashlib.sha256(file.read_bytes()).hexdigest()
            for file in directory.rglob("*")
            if file.is_file()
        }
        if actual != package["files"]:
            changed = sorted(
                name
                for name in actual.keys() | package["files"].keys()
                if actual.get(name) != package["files"].get(name)
            )
            raise SystemExit(f"Vendor content differs from the ledger: {directory.name}: {changed}")
    if {directory.name for directory in vendor.iterdir() if directory.is_dir()} != expected_packages:
        raise SystemExit("The vendor package list differs from the ledger")
    if re.search(r'^source\s*=\s*"git\+', (root / "Cargo.lock").read_text(encoding="utf-8"), re.M):
        raise SystemExit("Cargo.lock contains an upstream Git source")
    if args.ledger_only:
        print(f"Verified {len(expected_packages)} local packages and no Git sources in Cargo.lock.")
        return
    result = subprocess.run(
        [
            "cargo", "metadata", "--frozen", "--format-version", "1",
            "--no-default-features", "--features", "warp/warp_lite",
        ],
        cwd=root,
        check=True,
        capture_output=True,
        encoding="utf-8",
    )
    metadata = json.loads(result.stdout)
    git_sources = [
        package["name"] for package in metadata["packages"]
        if (package.get("source") or "").startswith("git+")
    ]
    if git_sources:
        raise SystemExit(f"Unlocalized Git dependencies: {git_sources}")
    members = set(metadata["workspace_members"])
    if any(
        package["id"] in members
        and Path(package["manifest_path"]).resolve().is_relative_to(vendor.resolve())
        for package in metadata["packages"]
    ):
        raise SystemExit("Vendored packages must be excluded from workspace membership")
    print(f"Verified {len(expected_packages)} local packages; Cargo resolved zero Git dependencies.")


if __name__ == "__main__":
    main()
