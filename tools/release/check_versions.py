#!/usr/bin/env python3
import argparse
import re
import sys
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def read_toml(path: Path) -> dict:
    return tomllib.loads(path.read_text())


def read_workspace_version() -> str:
    cargo = read_toml(ROOT / "Cargo.toml")
    return cargo["workspace"]["package"]["version"]


def read_module_version() -> str:
    module = (ROOT / "MODULE.bazel").read_text()
    match = re.search(r'version\s*=\s*"([^"]+)"', module)
    if match is None:
        raise RuntimeError("MODULE.bazel version not found")
    return match.group(1)


def read_header_version() -> str:
    header = (ROOT / "bindings/c/include/material_color/material_color.h").read_text()
    match = re.search(r'#define MATERIAL_COLOR_VERSION "([^"]+)"', header)
    if match is None:
        raise RuntimeError("C header version macro not found")
    return match.group(1)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tag", help="Expected release tag, for example v1.0.0")
    args = parser.parse_args()

    version = read_workspace_version()
    pyproject = read_toml(ROOT / "crates/material_color_py/pyproject.toml")
    versions = {
        "Cargo.toml": version,
        "MODULE.bazel": read_module_version(),
        "material_color.h": read_header_version(),
        "pyproject.toml": pyproject["project"]["version"],
    }

    mismatches = {
        source: found
        for source, found in versions.items()
        if found != version
    }
    if mismatches:
        for source, found in mismatches.items():
            print(f"{source}: expected {version}, found {found}", file=sys.stderr)
        return 1

    if args.tag is not None and args.tag != f"v{version}":
        print(f"tag {args.tag} does not match version {version}", file=sys.stderr)
        return 1

    print(version)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
