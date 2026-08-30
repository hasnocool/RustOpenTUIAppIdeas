# RustOpenTUIAppIdeas

A **Rust application idea incubator and software exploration laboratory** for discovering, researching, designing, prototyping, benchmarking, and graduating applications across terminal, desktop, web, embedded, services, libraries, simulations, games, developer tools, AI systems, and other Rust-friendly domains.

OpenTUI remains an important **UI and interaction laboratory**, but it is deliberately **not the boundary of the project**. An idea does not need to be a TUI to belong here, and an experiment may ultimately use a TUI, desktop UI, web UI, GUI, API, daemon, library, embedded interface, or no graphical interface at all.

OpenTUI's upstream project currently provides a native Zig core with TypeScript bindings. This repository therefore treats OpenTUI as one possible interaction/rendering technology rather than assuming a fixed Rust OpenTUI API. Renderer, frontend, backend, and deployment choices are recorded per experiment.

## Purpose

This is intentionally **not one giant application** and not merely a collection of TUI mockups. It is a laboratory where software concepts can be:

- captured
- categorized
- researched
- compared
- visually designed
- prototyped
- benchmarked
- simulated
- integrated with real systems
- abandoned
- revived
- combined with other ideas
- promoted into standalone projects

The central catalog is [`IDEAS.md`](IDEAS.md). Every substantial application idea can have its own directory under `apps/` with focused documentation covering the problem, users, interfaces, architecture, scope, experiments, milestones, risks, and expansion opportunities.

## Interface is a choice, not a constraint

The project uses a **domain-first** approach:

```text
                         IDEA
                          │
                          ▼
                    DOMAIN MODEL
                          │
             ┌────────────┼────────────┐
             ▼            ▼            ▼
          TERMINAL      DESKTOP       WEB
             │            │            │
             ▼            ▼            ▼
           OpenTUI       GUI          WASM
             │            │            │
             └────────────┼────────────┘
                          │
                          ▼
                     APPLICATION
                          │
             ┌────────────┼────────────┐
             ▼            ▼            ▼
          SERVICE      LIBRARY      EMBEDDED
```

A prototype should choose the simplest interface that helps answer the current question. A TUI may be perfect for an operations dashboard, while a compiler visualizer might eventually need a desktop/web graph view, and a sensor project might need a serial protocol plus a small terminal monitor.

## What belongs here?

Examples include:

- CLI and TUI applications
- desktop applications
- web applications and WASM experiments
- background services and daemons
- APIs and network services
- developer tooling
- databases and storage engines
- data-processing tools
- AI/LLM applications and agent systems
- simulations and games
- embedded and IoT software
- networking and distributed-system experiments
- observability and infrastructure tools
- compilers, interpreters, parsers, and language tooling
- graphics and multimedia experiments
- reusable Rust libraries and crates
- benchmarks and performance laboratories
- experiments that combine several of these areas

## Repository map

```text
.
├── IDEAS.md
├── README.md
├── CONTRIBUTING.md
├── ROADMAP.md
├── docs/
│   ├── ARCHITECTURE.md
│   ├── IDEA_LIFECYCLE.md
│   ├── TAXONOMY.md
│   ├── RUST_ECOSYSTEM_IDEAS.md
│   ├── OPENTUI_NOTES.md
│   ├── EXPERIMENTS.md
│   ├── QUALITY.md
│   └── templates/APP_README.md
├── apps/
│   ├── ai/
│   ├── cli/
│   ├── compiler/
│   ├── data/
│   ├── database/
│   ├── development/
│   ├── devops/
│   ├── desktop/
│   ├── embedded/
│   ├── fun/
│   ├── games/
│   ├── graphics/
│   ├── multimedia/
│   ├── networking/
│   ├── observability/
│   ├── offgrid/
│   ├── operations/
│   ├── productivity/
│   ├── security/
│   ├── simulation/
│   ├── systems/
│   ├── tui/
│   └── web/
└── experiments/
```

Directories may be created lazily. The taxonomy describes the intended universe; it does not require every category to contain code immediately.

## Principles

1. **Ideas before implementation.** Capture the problem and hypothesis before writing substantial code.
2. **Domain before interface.** Do not make TUI, GUI, web, or CLI requirements distort the underlying model.
3. **Small before large.** Start with the smallest experiment capable of disproving or validating the idea.
4. **Use Rust where Rust adds value.** Performance, safety, concurrency, portability, systems access, and reliability are useful reasons; Rust itself is not the product requirement.
5. **Terminal-first when appropriate.** OpenTUI is a preferred laboratory for interactive prototypes, not a mandatory final UI.
6. **Async, non-blocking I/O by default.** Slow filesystem, network, process, and integration work belongs outside the render/input loop.
7. **Measure before optimizing.** Benchmark meaningful workloads instead of optimizing assumptions.
8. **Prefer real experiments over speculative frameworks.** Build evidence before extracting abstractions.
9. **Failed experiments are valuable.** Document why an approach failed and what was learned.
10. **Interfaces can diverge.** A project may start as a TUI and graduate as a desktop/web/service application.
11. **Projects can combine.** Related experiments may merge into larger systems when their domain models genuinely fit.
12. **Graduation is encouraged.** Mature ideas should leave the incubator when they have a clear independent identity.

## Workflow

1. Browse `IDEAS.md`.
2. Select a domain and idea.
3. Create or open its application directory.
4. Define the smallest useful experiment.
5. Document the problem, users, constraints, and success criteria.
6. Research unknowns.
7. Choose the simplest appropriate interface and runtime architecture.
8. Prototype a vertical slice.
9. Record benchmarks, screenshots/ASCII diagrams, decisions, and failures.
10. Expand only where evidence supports it.
11. Promote, combine, continue, archive, or reject the idea explicitly.
12. Graduate mature projects into standalone repositories when appropriate.

See `docs/IDEA_LIFECYCLE.md`, `docs/ARCHITECTURE.md`, and `CONTRIBUTING.md`.
