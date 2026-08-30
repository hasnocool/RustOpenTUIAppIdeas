# Master Idea Catalog

This is the canonical inventory for the incubator. App directories are the source of detailed design notes; this file is the map, prioritization surface, and idea backlog.

## Status vocabulary

| Status | Meaning |
|---|---|
| `seed` | Captured idea; little research completed |
| `exploring` | UX/technical questions actively being investigated |
| `prototype` | A vertical slice is being implemented |
| `validated` | Core concept demonstrated successfully |
| `incubating` | Worth expanding, but still belongs in this repository |
| `graduating` | Ready for a standalone repository |
| `paused` | Intentionally parked |
| `archived` | Kept for historical/reference value |
| `rejected` | Tested and intentionally not pursued |

## Scoring model

Each idea can be scored from 1–5 for:

- **Utility** — recurring real-world usefulness
- **Learning** — how much it teaches Rust/TUI architecture
- **Feasibility** — likelihood of a small first version
- **Differentiation** — room for a distinctive experience
- **Expansion** — plausible path from toy to serious tool
- **Reuse** — reusable components/knowledge for other apps

`Priority = Utility + Learning + Feasibility + Differentiation + Expansion + Reuse`.

Scores are deliberately subjective. Their purpose is comparison, not fake precision.

---

## System & host tools

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| SYS-001 | SysPeek — live CPU/RAM/disk/network dashboard | seed | 25/30 | `apps/system/syspeek/` |
| SYS-002 | ProcView — interactive process explorer | seed | 24/30 | `apps/system/procview/` |
| SYS-003 | DiskLens — directory/storage explorer | seed | 25/30 | `apps/system/disklens/` |
| SYS-004 | PortWatch — listening-port and process inspector | seed | 22/30 | `apps/system/portwatch/` |
| SYS-005 | LogView — searchable streaming log viewer | seed | 27/30 | `apps/system/logview/` |

## Development tools

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| DEV-001 | GitDash — interactive Git repository cockpit | seed | 28/30 | `apps/development/gitdash/` |
| DEV-002 | RepoTUI — repository/project portfolio browser | seed | 27/30 | `apps/development/repotui/` |
| DEV-003 | EnvTUI — environment/config inspector | seed | 20/30 | `apps/development/envtui/` |
| DEV-004 | Markdown Explorer — keyboard-driven Markdown reader | seed | 24/30 | `apps/development/markdown-explorer/` |

## AI / LLM tools

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| AI-001 | LLM-TUI — multi-backend terminal chat | exploring | 29/30 | `apps/ai/llm-tui/` |
| AI-002 | ModelManager — local model inventory and lifecycle UI | exploring | 28/30 | `apps/ai/model-manager/` |
| AI-003 | AI Router — route tasks across local/remote models | seed | 30/30 | `apps/ai/ai-router/` |
| AI-004 | ModelBench — repeatable local model benchmark explorer | seed | 29/30 | `apps/ai/modelbench/` |

## Operations / infrastructure

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| OPS-001 | DockerTUI — container/service manager | seed | 28/30 | `apps/operations/dockertui/` |
| OPS-002 | NetDash — interfaces, routes, traffic and connectivity | seed | 25/30 | `apps/operations/netdash/` |
| OPS-003 | ServiceBoard — systemd/service health dashboard | seed | 25/30 | `apps/operations/serviceboard/` |

## Off-grid / energy

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| OFF-001 | SolarTUI — solar/battery/load dashboard | exploring | 29/30 | `apps/offgrid/solartui/` |
| OFF-002 | PowerForecast — energy production/consumption forecast | seed | 27/30 | `apps/offgrid/powerforecast/` |

## Data / information tools

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| DATA-001 | JSON Explorer — interactive tree browser | seed | 26/30 | `apps/data/json-explorer/` |
| DATA-002 | CSV Explorer — spreadsheet-like terminal data browser | seed | 28/30 | `apps/data/csv-explorer/` |
| DATA-003 | HardwareBench — hardware/LLM benchmark research browser | exploring | 30/30 | `apps/data/hardwarebench/` |

## Productivity

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| PROD-001 | TaskForge — keyboard-first task manager | seed | 23/30 | `apps/productivity/taskforge/` |
| PROD-002 | NoteTUI — local Markdown notes browser | seed | 25/30 | `apps/productivity/notetui/` |
| PROD-003 | Pomodoro TUI — timer and work-session tracker | seed | 20/30 | `apps/productivity/pomodoro/` |

## Fun / visual experiments

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| FUN-001 | ASCII Weather — expressive terminal weather display | seed | 21/30 | `apps/fun/ascii-weather/` |
| FUN-002 | QR TUI — terminal QR generation/preview experiment | seed | 22/30 | `apps/fun/qr-tui/` |

---

## Cross-cutting idea backlog

These are capabilities that may become apps, reusable crates, or experiments rather than standalone applications:

- Command palette framework
- Async event bus for TUIs
- Non-blocking subprocess supervisor
- Terminal capability detector abstraction
- Theme system with light/dark/high-contrast presets
- Responsive layout rules for small terminals
- Unicode/grapheme-aware text measurement
- Virtualized lists and tables
- Fuzzy search component
- Modal/overlay manager
- Notification/toast system
- Status-bar framework
- Persistent local configuration format
- Application state snapshotting
- TUI telemetry/profiling overlay
- Snapshot/golden rendering tests
- Accessibility-oriented keyboard navigation model
- Plugin architecture for data providers
- Shared benchmark result schema
- Idea catalog browser TUI

## Future idea intake

New ideas should initially be added here with:

```text
ID
Name
One-sentence purpose
Category
Status = seed
Initial score
Directory
```

Then create the app directory from `docs/templates/APP_README.md` and expand the concept there.

## Promotion rule

An idea should normally graduate only after it has a demonstrated vertical slice, documented UX, known technical risks, and a clear reason to exist independently. A high score alone is not enough.
