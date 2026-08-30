# scripts/validate_ascii_diagrams.py
"""Validate ASCII/terminal diagrams used by the idea incubator.

The repository treats diagrams as documentation artifacts. This validator checks
Markdown fenced text diagrams for basic structural quality and reports overly
wide lines that are likely to wrap in an 80-column terminal.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCAN_ROOTS = (ROOT / "docs", ROOT / "apps", ROOT / "incubator")
FENCE_RE = re.compile(r"```(?:text|ascii|ansi|terminal|console)?\n(.*?)```", re.DOTALL)


def iter_markdown() -> list[Path]:
    files: list[Path] = []
    for root in SCAN_ROOTS:
        if root.exists():
            files.extend(root.rglob("*.md"))
    return sorted(set(files))


def validate_file(path: Path) -> tuple[int, list[str]]:
    text = path.read_text(encoding="utf-8")
    blocks = FENCE_RE.findall(text)
    warnings: list[str] = []

    for index, block in enumerate(blocks, start=1):
        lines = block.rstrip("\n").splitlines()
        if not any(line.strip() for line in lines):
            warnings.append(f"diagram {index} is empty")
            continue

        if max(map(len, lines), default=0) > 100:
            warnings.append(f"diagram {index} contains a line wider than 100 columns")

        if any("\t" in line for line in lines):
            warnings.append(f"diagram {index} contains tabs; use spaces for stable alignment")

    return len(blocks), warnings


def main() -> int:
    total = 0
    failures = 0

    for path in iter_markdown():
        count, warnings = validate_file(path)
        if count:
            total += count
        for warning in warnings:
            failures += 1
            print(f"ERROR: {path.relative_to(ROOT)}: {warning}")

    print(f"Validated {total} ASCII/terminal diagram blocks.")
    if failures:
        print(f"Found {failures} diagram quality issue(s).")
        return 1

    print("ASCII diagram validation passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
