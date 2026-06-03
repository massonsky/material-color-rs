#!/usr/bin/env python3
import argparse
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
LIB_SUFFIXES = {
    ".a",
    ".dll",
    ".dylib",
    ".lib",
    ".so",
}


def collect_libraries(profile: str) -> list[Path]:
    target_dir = ROOT / "target" / profile
    libraries = []
    for path in target_dir.iterdir():
        if "material_color_capi" not in path.name:
            continue
        if path.suffix in LIB_SUFFIXES:
            libraries.append(path)
    return sorted(libraries)


def add_file(archive: zipfile.ZipFile, source: Path, archive_name: str) -> None:
    archive.write(source, archive_name)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", required=True)
    parser.add_argument("--target-label", required=True)
    parser.add_argument("--output-dir", default="dist")
    parser.add_argument("--profile", default="release")
    args = parser.parse_args()

    libraries = collect_libraries(args.profile)
    if not libraries:
        raise RuntimeError("material_color_capi libraries were not found in target/release")

    output_dir = ROOT / args.output_dir
    output_dir.mkdir(parents=True, exist_ok=True)
    archive_path = output_dir / (
        f"material-color-capi-{args.version}-{args.target_label}.zip"
    )

    with zipfile.ZipFile(archive_path, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        add_file(
            archive,
            ROOT / "bindings/c/include/material_color/material_color.h",
            "include/material_color/material_color.h",
        )
        add_file(
            archive,
            ROOT / "bindings/cpp/include/material_color/material_color.hpp",
            "include/material_color/material_color.hpp",
        )
        add_file(archive, ROOT / "README.md", "README.md")
        add_file(archive, ROOT / "CHANGELOG.md", "CHANGELOG.md")
        add_file(archive, ROOT / "docs/RELEASE.md", "docs/RELEASE.md")
        for library in libraries:
            add_file(archive, library, f"lib/{library.name}")

    print(archive_path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
