# scripts/validate_catalog.py
"""Validate that the master idea catalog and app documentation stay synchronized."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
IDEAS = ROOT / "IDEAS.md"
APPS = ROOT / "apps"

PATH_PATTERN = re.compile(r"`(apps/[^`]+/)`")
ID_PATTERN = re.compile(r"\|\s*([A-Z]+-\d{3})\s*\|")


def main() -> int:
    errors: list[str] = []
    text = IDEAS.read_text(encoding="utf-8")

    catalog_ids: set[str] = set()
    for match in ID_PATTERN.finditer(text):
        idea_id = match.group(1)
        if idea_id in catalog_ids:
            errors.append(f"duplicate idea id: {idea_id}")
        catalog_ids.add(idea_id)

    catalog_paths = PATH_PATTERN.findall(text)
    if len(catalog_paths) != len(set(catalog_paths)):
        errors.append("duplicate app path in IDEAS.md")

    for relative in catalog_paths:
        directory = ROOT / relative
        readme = directory / "README.md"
        if not directory.is_dir():
            errors.append(f"missing app directory: {relative}")
        elif not readme.is_file():
            errors.append(f"missing app README: {relative}README.md")

    if not APPS.is_dir():
        errors.append("missing apps/ directory")

    if errors:
        print("Catalog validation failed:")
        for error in errors:
            print(f"- {error}")
        return 1

    print(f"Catalog OK: {len(catalog_ids)} ideas, {len(catalog_paths)} documented app paths")
    return 0


if __name__ == "__main__":
    sys.exit(main())
