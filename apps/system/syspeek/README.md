# SYS-001 — SysPeek

**Status:** seed  
**Purpose:** A compact live dashboard for CPU, memory, disks, network, uptime, and load.

## Why it belongs in the incubator

SysPeek is a foundational learning project for refresh loops, process/system APIs, tables, sparklines, resize handling, and non-blocking telemetry collection.

## MVP

- Host summary
- CPU and memory utilization
- Filesystem capacity
- Network RX/TX rates
- Refresh interval control
- Quit/help overlay

## UX

```text
┌─ HOST ─────────────────────────────────────────────┐
│ uptime  3d 04h       load  1.82 1.71 1.54         │
├─ CPU ───────────────────────────────────────────────┤
│ ███████████████░░░░░  73%                          │
├─ MEMORY ────────────────────────────────────────────┤
│ ██████████░░░░░░░░░  51%                           │
├─ NETWORK ──────────────────────────────────────────┤
│ ↓ 42 MB/s                    ↑ 8 MB/s              │
└─────────────────────────────────────────────────────┘
```

## Architecture experiment

A telemetry worker samples metrics on a timer and sends immutable snapshots to the UI. Rendering never queries slow system APIs directly.

## Stretch ideas

Per-process drill-down, historical graphs, alerts, sensor temperatures, GPU metrics, export, and remote hosts.

## Risks

Cross-platform metric APIs and GPU telemetry vary considerably.

## Graduation

Graduate if the dashboard becomes a dependable daily system-monitoring tool rather than only a learning demo.
