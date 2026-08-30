# PROD-002 — NoteTUI

**Status:** seed  
**Purpose:** Local Markdown notes browser/editor optimized for fast keyboard navigation.

## MVP

- Notes directory browser
- Markdown preview
- Search
- Create/edit/delete with confirmation
- Tags/front matter optional
- Local persistence

## UX

```text
NOTES

> projects/rust-tui.md
  ideas/llm-router.md
  research/hardware.md
  journal/2026-08-29.md

────────────────────────────────────────
# Rust TUI Ideas

A note about the experiment...
```

## Architecture

Separate note storage, indexing, and editing from rendering. Large-file indexing must run in the background. Text editing should use grapheme-aware cursor logic.

## Stretch

Backlinks, full-text index, graph view, Git history, templates, encrypted storage where appropriate, and external editor integration.
