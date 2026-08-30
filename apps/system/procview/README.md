# SYS-002 — ProcView

**Status:** seed  
**Purpose:** Interactive process explorer with sorting, filtering, inspection, and safe lifecycle actions.

## MVP

- Process table with PID, CPU, memory, state, command
- Sort columns
- Fuzzy process search
- Process detail panel
- Refresh without blocking
- Graceful quit

## UX

```text
PID     CPU     MEM     STATE    COMMAND
> 1842  32.1%   4.2%    running  ollama
  2931  12.0%   2.1%    sleep    docker
  4120   4.8%   0.8%    running  firefox
```

## Technical experiment

Compare polling intervals and process enumeration costs. Keep process collection in a worker and send snapshots to the renderer.

## Stretch

Tree view, per-process I/O, open files, CPU affinity, historical sparklines, and remote process inspection.

## Risk

Process metadata and lifecycle controls differ by operating system.

## Graduation

A stable cross-platform process explorer with excellent keyboard navigation and low overhead.
