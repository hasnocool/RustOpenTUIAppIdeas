# SYS-003 — DiskLens

**Status:** seed  
**Purpose:** Explore filesystem usage interactively instead of waiting for a giant command-line report.

## MVP

- Scan a selected directory
- Show child sizes and item counts
- Sort by size/name/type
- Drill into directories
- Ignore configurable paths
- Cancel scans cleanly

## UX

```text
DISK LENS  /home

> projects/          84.2 GB   ███████████████
  media/             41.8 GB   ███████
  Downloads/         12.4 GB   ██
  .cache/             8.1 GB   █

Enter=open  Backspace=up  s=sort  /=filter  c=cancel
```

## Architecture experiment

Scanning must be a cancellable background job. UI receives progress and completed directory snapshots. Avoid loading millions of entries into memory unnecessarily.

## Stretch

Treemap-like terminal visualization, duplicate detection, age filters, file-type analytics, cleanup suggestions, and export.

## Risk

Filesystem permissions, symlinks, mount boundaries, and huge directories need careful handling.
