#!/usr/bin/env python3
"""Set [package].version in Cargo.toml from a release tag."""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

VERSION_LINE_RE = re.compile(r'^version\s*=\s*"[^"]*"\s*$')


def fail(message: str) -> "NoReturn":
    print(message)
    raise SystemExit(1)


def parse_tag_version(raw_tag: str) -> str:
    tag = raw_tag.strip()
    if tag.startswith("refs/tags/"):
        tag = tag.removeprefix("refs/tags/")

    version = tag[1:] if tag.startswith("v") else tag
    if not version:
        fail("Failed to read release tag version")
    return version


def update_cargo_version(cargo_path: Path, version: str) -> None:
    try:
        lines = cargo_path.read_text(encoding="utf-8").splitlines(keepends=True)
    except OSError:
        fail("Failed to update package version in Cargo.toml")

    in_package = False
    updated = False

    for idx, line in enumerate(lines):
        stripped = line.strip()

        if stripped == "[package]":
            in_package = True
            continue

        if in_package and stripped.startswith("[") and stripped != "[package]":
            in_package = False

        if in_package and VERSION_LINE_RE.match(stripped):
            newline = "\r\n" if line.endswith("\r\n") else "\n"
            lines[idx] = f'version = "{version}"{newline}'
            updated = True
            break

    if not updated:
        fail("Failed to locate package version in Cargo.toml")

    tmp_path = cargo_path.with_suffix(cargo_path.suffix + ".tmp")
    try:
        tmp_path.write_text("".join(lines), encoding="utf-8")
        os.replace(tmp_path, cargo_path)
    except OSError:
        if tmp_path.exists():
            tmp_path.unlink(missing_ok=True)
        fail("Failed to update package version in Cargo.toml")

    verify_version = None
    in_package = False
    for line in lines:
        stripped = line.strip()
        if stripped == "[package]":
            in_package = True
            continue
        if in_package and stripped.startswith("[") and stripped != "[package]":
            in_package = False
        if in_package and VERSION_LINE_RE.match(stripped):
            verify_version = stripped
            break

    if verify_version != f'version = "{version}"':
        fail("Cargo.toml version update verification failed")


def main() -> None:
    if len(sys.argv) < 2:
        fail("Failed to read release tag version")

    version = parse_tag_version(sys.argv[1])
    cargo_path = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("Cargo.toml")
    update_cargo_version(cargo_path, version)


if __name__ == "__main__":
    main()
