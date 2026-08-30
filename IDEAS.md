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

Each idea can be scored from 1–5 for Utility, Learning, Feasibility, Differentiation, Expansion, and Reuse.

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
| DEV-005 | CargoLab — interactive Cargo project explorer | seed | 27/30 | `apps/development/cargo-lab/` |
| DEV-006 | DepGraph — dependency graph explorer | seed | 27/30 | `apps/development/depgraph/` |
| DEV-007 | CrateFinder — local Cargo ecosystem search | seed | 24/30 | `apps/development/crate-finder/` |
| DEV-008 | RustSize — Rust binary/project size analyzer | seed | 26/30 | `apps/development/rustsize/` |
| DEV-009 | APIProbe — interactive HTTP API explorer | seed | 28/30 | `apps/development/apiprobe/` |
| DEV-010 | EnvDiff — compare environment/configuration sets | seed | 25/30 | `apps/development/envdiff/` |
| DEV-011 | DiffScope — generalized visual diff explorer | seed | 29/30 | `apps/development/diffscope/` |

## AI / LLM tools

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| AI-001 | LLM-TUI — multi-backend terminal chat | exploring | 29/30 | `apps/ai/llm-tui/` |
| AI-002 | ModelManager — local model inventory and lifecycle UI | exploring | 28/30 | `apps/ai/model-manager/` |
| AI-003 | AI Router — route tasks across local/remote models | seed | 30/30 | `apps/ai/ai-router/` |
| AI-004 | ModelBench — repeatable local model benchmark explorer | seed | 29/30 | `apps/ai/modelbench/` |
| AI-005 | PromptLab — interactive prompt experimentation workspace | seed | 28/30 | `apps/ai/promptlab/` |
| AI-006 | TokenLens — prompt/token/context inspection tool | seed | 27/30 | `apps/ai/tokenlens/` |
| AI-007 | ModelMatrix — multi-model capability comparison | seed | 28/30 | `apps/ai/modelmatrix/` |
| AI-008 | ContextTUI — interactive context-window visualizer | seed | 27/30 | `apps/ai/context-tui/` |

## Operations / infrastructure

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| OPS-001 | DockerTUI — container/service manager | seed | 28/30 | `apps/operations/dockertui/` |
| OPS-002 | NetDash — interfaces, routes, traffic and connectivity | seed | 25/30 | `apps/operations/netdash/` |
| OPS-003 | ServiceBoard — systemd/service health dashboard | seed | 25/30 | `apps/operations/serviceboard/` |
| OPS-004 | WatchTUI — generic non-blocking command watcher | seed | 26/30 | `apps/operations/watchtui/` |
| OPS-005 | PortMap — visual local port/process map | seed | 27/30 | `apps/operations/portmap/` |
| OPS-006 | ServiceMap — service dependency graph | seed | 28/30 | `apps/operations/service-map/` |
| OPS-007 | ProcessTree — hierarchical process explorer | seed | 26/30 | `apps/operations/process-tree/` |
| OPS-008 | LatencyTUI — interactive network latency monitor | seed | 25/30 | `apps/operations/latency-tui/` |

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
| DATA-004 | TreeTUI — universal hierarchical data explorer | seed | 28/30 | `apps/data/tree-tui/` |
| DATA-005 | DataProbe — unknown-file/data structure investigator | seed | 29/30 | `apps/data/dataprobe/` |
| DATA-006 | JSONPatchTUI — interactive JSON difference viewer | seed | 28/30 | `apps/data/jsonpatch-tui/` |
| DATA-007 | HistogramTUI — terminal histogram explorer | seed | 23/30 | `apps/data/histogram-tui/` |
| DATA-008 | SparkTUI — compact terminal chart/series explorer | seed | 25/30 | `apps/data/spark-tui/` |
| DATA-009 | TimelineTUI — interactive event and project timelines | seed | 26/30 | `apps/data/timeline-tui/` |

## Productivity

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| PROD-001 | TaskForge — keyboard-first task manager | seed | 23/30 | `apps/productivity/taskforge/` |
| PROD-002 | NoteTUI — local Markdown notes browser | seed | 25/30 | `apps/productivity/notetui/` |
| PROD-003 | Pomodoro TUI — timer and work-session tracker | seed | 20/30 | `apps/productivity/pomodoro/` |
| PROD-004 | KanbanTUI — keyboard-driven terminal Kanban board | seed | 27/30 | `apps/productivity/kanban-tui/` |
| PROD-005 | DecisionTUI — weighted decision matrix | seed | 29/30 | `apps/productivity/decision-tui/` |

## Fun / visual experiments

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| FUN-001 | ASCII Weather — expressive terminal weather display | seed | 21/30 | `apps/fun/ascii-weather/` |
| FUN-002 | QR TUI — terminal QR generation/preview experiment | seed | 22/30 | `apps/fun/qr-tui/` |
| FUN-003 | ColorLab — ANSI terminal color/theme laboratory | seed | 25/30 | `apps/fun/colorlab/` |
| FUN-004 | KeyboardLab — interactive keyboard/keybinding designer | seed | 26/30 | `apps/fun/keyboard-lab/` |

## Tiny tools / focused experiments

These are intentionally small applications that can be built quickly and used as component laboratories for larger projects.

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| TINY-001 | ASCII Architect — interactive ASCII diagram builder | seed | 30/30 | `apps/tiny-tools/ascii-architect/` |
| TINY-002 | ASCII Flow — interactive terminal flowchart designer | seed | 29/30 | `apps/tiny-tools/ascii-flow/` |
| TINY-003 | StateMachineTUI — finite-state-machine designer/exporter | seed | 30/30 | `apps/tiny-tools/state-machine-tui/` |
| TINY-004 | RegexLab — interactive regex playground | seed | 28/30 | `apps/tiny-tools/regexlab/` |
| TINY-005 | HexViewTUI — interactive binary/hex viewer | seed | 27/30 | `apps/tiny-tools/hexview-tui/` |
| TINY-006 | BenchmarkTUI — repeatable operation benchmark dashboard | seed | 28/30 | `apps/tiny-tools/benchmark-tui/` |
| TINY-007 | Context-free Watch — programmable command monitor | seed | 25/30 | `apps/tiny-tools/watchtui/` |

## Advanced applications / systems

These ideas intentionally move beyond small utilities. They are multi-component applications intended to exercise persistent state, asynchronous workers, event streams, graphs, databases, plugins, simulation, orchestration, or external integrations while remaining approachable as staged Rust/OpenTUI projects.

### AI & agent systems

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| ADV-AI-001 | AI Command Center — terminal AI workstation for models, tasks, agents, jobs, costs, memory, and providers | seed | 30/30 | `apps/advanced/ai-command-center/` |
| ADV-AI-002 | Agent Laboratory — interactive multi-agent planning, delegation, tool, memory, and evaluation environment | seed | 30/30 | `apps/advanced/agent-laboratory/` |
| ADV-AI-003 | AI Workflow Engine — visual/task-oriented AI and automation workflow runner | seed | 29/30 | `apps/advanced/ai-workflow-engine/` |
| ADV-AI-004 | Knowledge Graph TUI — navigable graph connecting projects, notes, documents, concepts, and sources | seed | 29/30 | `apps/advanced/knowledge-graph-tui/` |

### Infrastructure & networking

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| ADV-INF-001 | Infrastructure Command Center — hosts, containers, services, storage, network, and logs in one TUI | seed | 30/30 | `apps/advanced/infrastructure-command-center/` |
| ADV-INF-002 | Network Observatory — live topology, latency, routes, ports, traffic, and node inspection | seed | 29/30 | `apps/advanced/network-observatory/` |
| ADV-INF-003 | Event Stream Explorer — interactive event-stream inspection, filtering, replay, and consumer tracing | seed | 28/30 | `apps/advanced/event-stream-explorer/` |
| ADV-INF-004 | Web Crawler Observatory — crawler jobs, queues, workers, domains, throughput, errors, and crawl graphs | seed | 28/30 | `apps/advanced/web-crawler-observatory/` |

### Data & research systems

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| ADV-DATA-001 | Data Observatory — terminal data analysis workspace for files, databases, APIs, transformations, and charts | seed | 30/30 | `apps/advanced/data-observatory/` |
| ADV-DATA-002 | SQLite Studio TUI — database browser, SQL editor, schema explorer, query history, and results viewer | seed | 29/30 | `apps/advanced/sqlite-studio/` |
| ADV-DATA-003 | Research Observatory — sources, papers, notes, claims, topics, evidence, and knowledge graphs | seed | 30/30 | `apps/advanced/research-observatory/` |
| ADV-DATA-004 | Project Universe — portfolio graph for repositories, projects, dependencies, activity, health, issues, and PRs | seed | 29/30 | `apps/advanced/project-universe/` |

### Simulation & optimization

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| ADV-SIM-001 | Simulation Laboratory — reusable simulation environment with visualization, controls, metrics, and scenario management | seed | 30/30 | `apps/advanced/simulation-laboratory/` |
| ADV-SIM-002 | Colony Simulator — agent-based bee/ant colony ecosystem simulation | seed | 29/30 | `apps/advanced/colony-simulator/` |
| ADV-SIM-003 | Power Grid Simulator — energy generation, storage, loads, weather, and grid-flow simulation | seed | 30/30 | `apps/advanced/power-grid-simulator/` |
| ADV-SIM-004 | Optimization Laboratory — interactive scheduling, routing, packing, energy, and resource-allocation experiments | seed | 29/30 | `apps/advanced/optimization-laboratory/` |

### Developer platforms

| ID | Idea | Status | Score | Directory |
|---|---|---:|---:|---|
| ADV-DEV-001 | Developer Cockpit — project, Git, tasks, builds, tests, logs, terminals, and workspace state | seed | 30/30 | `apps/advanced/developer-cockpit/` |
| ADV-DEV-002 | TUI Component Laboratory — interactive showcase, benchmark, and testbed for reusable OpenTUI components | seed | 30/30 | `apps/advanced/tui-component-laboratory/` |
| ADV-DEV-003 | Dependency Universe — generalized dependency graph explorer with cycles, duplicates, versions, size, and licenses | seed | 28/30 | `apps/advanced/dependency-universe/` |
| ADV-DEV-004 | Workflow Engine — generic event-driven automation graph with triggers, workers, branching, retries, and persistence | seed | 30/30 | `apps/advanced/workflow-engine/` |

## Advanced project architecture patterns

The advanced tier should reuse a small number of architectural patterns rather than creating unrelated one-off implementations.

```text
                         ADVANCED TUI APPLICATION
                                    |
             +----------------------+----------------------+
             |                      |                      |
             v                      v                      v
          UI STATE              DOMAIN CORE           PERSISTENCE
             |                      |                      |
             v                      v                      v
        OPEN TUI VIEW          EVENTS / COMMANDS       SQLite / JSON
             |                      |                      |
             +----------+-----------+-----------+----------+
                        |                       |
                        v                       v
                 ASYNC WORKERS             INTEGRATIONS
                        |                       |
                 +------+-------+       +------+-------+
                 |              |       |              |
               FILES          NETWORK  GIT           APIs
```

### Multi-worker pattern

```text
                         EVENT LOOP
                             |
                    +--------+--------+
                    |                 |
                    v                 v
                 UI TASK          WORK QUEUE
                    |                 |
                    |          +------+------+------+
                    |          |      |      |      |
                    |          v      v      v      v
                    |        READ   FETCH  PARSE  COMPUTE
                    |          |      |      |      |
                    |          +------+------+------+
                    |                 |
                    +--------< EVENTS + RESULTS
                             |
                             v
                           RENDER
```

All long-running work should be asynchronous/non-blocking from the UI perspective. Workers communicate through channels/events and never perform blocking work directly in the render/input path.

### Graph application pattern

```text
                  GRAPH STORE
                       |
              +--------+--------+
              |                 |
             NODES            EDGES
              |                 |
              +--------+--------+
                       |
                       v
                GRAPH ENGINE
                       |
              +--------+--------+
              |        |        |
              v        v        v
            FILTER   LAYOUT   SEARCH
              |        |        |
              +--------+--------+
                       v
                 OPEN TUI VIEW
```

### Simulation pattern

```text
                    SCENARIO
                       |
                       v
                  SIMULATION
                       |
              +--------+--------+
              |        |        |
              v        v        v
            AGENTS   RULES    RESOURCES
              |        |        |
              +--------+--------+
                       |
                       v
                    TICK N
                       |
              +--------+--------+
              |                 |
              v                 v
           METRICS          SNAPSHOT
              |                 |
              +--------+--------+
                       v
                    RENDER
```

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
- ASCII diagram editor widget
- ASCII box/connector layout engine
- Diagram import/export abstraction
- State-machine graph widget
- Timeline widget
- Decision-matrix widget
- Histogram/sparkline chart widgets
- Generic tree explorer widget
- Interactive diff widget
- Keyboard-layout visualization widget
- ANSI color/theme preview widget
- Context-window/token budget visualization widget
- Graph layout engine
- Event-stream viewer widget
- Simulation tick/control framework
- Scenario/save-state format
- Worker pool abstraction
- Job queue abstraction
- Structured application event log
- Plugin/provider registry
- Data-source adapter interface
- Query editor component
- Schema browser component
- Terminal dashboard grid/layout engine
- TUI command execution sandbox
- Cross-application metrics model

## Incubator meta-project

The repository itself should eventually expose these ideas through an interactive TUI that discovers the catalog and application documentation at runtime. The catalog browser should support:

- category filtering
- status filtering
- fuzzy search
- score sorting
- application detail views
- ASCII diagram browsing
- architecture/state/navigation diagram views
- experiment and prototype discovery
- idea lifecycle updates
- documentation freshness indicators
- keyboard-driven navigation
- related-idea graph traversal
- reusable-component dependency discovery
- advanced-project architecture views
- experiment-to-project promotion tracking

The meta-project is intentionally both a useful application and a testbed for the shared components listed above.

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

## Advanced-project promotion ladder

```text
                         IDEA
                           |
                           v
                      ASCII DESIGN
                           |
                           v
                    DOMAIN PROTOTYPE
                           |
                           v
                    INTERACTIVE TUI
                           |
                           v
                  ASYNC / DATA LAYER
                           |
                           v
                 REAL INTEGRATION TEST
                           |
                           v
                     VALIDATION
                           |
                    +------+------+
                    |             |
                    v             v
                  INCUBATE     GRADUATE
                    |             |
                    v             v
                 ITERATE      STANDALONE
```

A complicated idea should be decomposed into independently testable vertical slices. The incubator should prefer proving one complete interaction path before expanding breadth.
