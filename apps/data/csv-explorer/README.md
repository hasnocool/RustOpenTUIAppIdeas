# DATA-002 — CSV Explorer

**Status:** seed  
**Purpose:** A terminal spreadsheet-like browser for CSV/TSV datasets.

## MVP

- Open CSV/TSV
- Detect headers and types
- Horizontal/vertical scrolling
- Column sorting
- Search/filter
- Basic summary statistics

## UX

```text
NAME              CPU      RAM       PRICE
──────────────────────────────────────────────
Ryzen 7840U       8C       32GB      $399
M2                8C       24GB      $599
N100              4C       16GB      $150
```

## Architecture

Use streaming or chunked reads for large files. Keep parsing and statistics work outside the UI loop. Virtualize visible rows and columns.

## Stretch

Computed columns, grouping, charts rendered in terminal, SQL-like filtering, export, and multi-file comparison.

## Graduation

Promote if it becomes a practical terminal data inspection tool for large datasets.
