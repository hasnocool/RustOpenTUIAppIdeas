# Incubator Architecture

The repository is an **application incubator**, not a TUI-only project. Its architecture deliberately separates the **domain/problem being explored** from the **interface used to explore it**.

## Architectural layers

```text
                         IDEA CATALOG
                              │
                              ▼
                       EXPERIMENT DEFINITION
                              │
                              ▼
                         DOMAIN MODEL
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
          SERVICES          LIBRARIES         DATA
             │                │                │
             └────────────────┼────────────────┘
                              │
                    ┌─────────┼─────────┐
                    ▼         ▼         ▼
                  TUI       DESKTOP     WEB
                    │         │         │
                    ▼         ▼         ▼
                 OpenTUI    GUI/WGPU   WASM
                    │         │         │
                    └─────────┼─────────┘
                              ▼
                       DEPLOYABLE APP
```

The important boundary is **domain ↔ interface**, not `TUI ↔ everything else`.

## Repository layers

### Catalog layer

```text
IDEAS.md
   │
   ├── domain/category
   │      └── application idea
   │             ├── README.md
   │             ├── SPEC.md
   │             ├── ARCHITECTURE.md
   │             └── ROADMAP.md
   │
   └── cross-cutting experiments
```

`IDEAS.md` answers **what exists**. An app README answers **why it exists**. `SPEC.md` answers **what it should do**. `ARCHITECTURE.md` answers **how the system may be decomposed**. `ROADMAP.md` answers **what happens next**.

### Implementation layer

A larger Rust application may use:

```text
src/
├── main.rs                 # startup/runtime lifecycle
├── lib.rs                  # reusable library boundary, when appropriate
├── app.rs                  # application orchestration
├── domain/                 # domain logic independent of UI
│   ├── models.rs
│   ├── rules.rs
│   └── services.rs
├── application/            # use cases / commands / workflows
├── adapters/               # external systems and protocol adapters
├── infrastructure/         # persistence, network, processes, hardware
├── ui/                     # optional interface layer
│   ├── tui/
│   ├── desktop/
│   └── web/
├── event.rs                # internal/event abstractions when useful
├── command.rs              # application commands
└── persistence/             # local state/config/cache
```

Do not force this exact structure on tiny experiments. Start small, but preserve a clean seam around domain logic whenever multiple interfaces or integrations are plausible.

## Interface strategy

A project can have multiple frontends over the same core:

```text
                     ┌─────────────┐
                     │ DOMAIN CORE │
                     └──────┬──────┘
                            │
           ┌────────────────┼────────────────┐
           ▼                ▼                ▼
      ┌─────────┐      ┌─────────┐      ┌─────────┐
      │   TUI   │      │ DESKTOP │      │   WEB   │
      └─────────┘      └─────────┘      └─────────┘
           │                │                │
        OpenTUI          WGPU/GUI          WASM
```

This is especially useful when the TUI is the fastest way to prototype an interaction, but a different interface is better for the eventual product.

## Event and concurrency model

Interactive applications should separate input, state transitions, rendering, and slow external work:

```text
                       USER / SYSTEM EVENTS
                               │
                               ▼
                        EVENT INGESTION
                               │
                               ▼
                        STATE / COMMANDS
                               │
                 ┌─────────────┼─────────────┐
                 ▼             ▼             ▼
              TUI UI       DESKTOP UI      WEB UI
                 │             │             │
                 └─────────────┼─────────────┘
                               │
                               ▼
                         DOMAIN COMMAND
                               │
                  ┌────────────┼────────────┐
                  ▼            ▼            ▼
               NETWORK       DISK        PROCESS
               WORKER       WORKER        WORKER
                  │            │            │
                  └────────────┼────────────┘
                               ▼
                         RESULT EVENTS
                               │
                               ▼
                         STATE UPDATE
```

Slow filesystem, network, subprocess, database, and hardware operations should not block interactive rendering or input handling. Prefer asynchronous, thread-safe, message-driven boundaries where the workload warrants them.

## Domain portability

When an idea may eventually have multiple interfaces, define domain types without UI-specific assumptions:

```text
GOOD

Domain Event
   │
   ├── TUI renderer
   ├── JSON API
   ├── WebSocket stream
   └── Desktop view

AVOID

Domain Event
   │
   └── directly constructs terminal cells
```

The domain should describe **what happened**. The interface decides **how it is presented**.

## Renderer boundary

OpenTUI can be used as the primary interactive laboratory where appropriate. Keep rendering behind a small interface or presentation module when doing so reduces coupling. This allows experiments to compare OpenTUI-compatible approaches, a Rust-native TUI framework, a desktop renderer, a web frontend, or a headless mode without rewriting the domain model.

## Headless mode

Important systems should consider a headless execution path:

```text
                 APPLICATION CORE
                        │
              ┌─────────┴─────────┐
              ▼                   ▼
          INTERACTIVE          HEADLESS
              │                   │
          TUI / GUI            CLI / API
              │                   │
              └─────────┬─────────┘
                        ▼
                    SAME CORE
```

Headless operation improves automation, testing, CI, scripting, benchmarking, and future service deployment.

## Persistence

Prefer boring, inspectable formats for prototypes: TOML, JSON, SQLite, or line-oriented files depending on the problem. A prototype should not require a complex database unless persistence is itself the experiment.

## Shared components

Extract shared components only after repeated evidence. A component should ideally have:

1. at least two meaningful consumers;
2. a stable API boundary;
3. documented behavior;
4. tests or rendering snapshots where applicable;
5. a reason to exist independently of one application's UI.

Reusable components may be UI widgets, domain crates, protocol clients, simulation engines, graph models, storage abstractions, or benchmarking utilities—not only TUI widgets.

## Graduation architecture

A successful incubator idea can graduate in several forms:

```text
                     INCUBATOR IDEA
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
      TUI PROJECT       RUST CRATE       SERVICE
          │                │                │
          ▼                ▼                ▼
       STANDALONE      PUBLISHED/        DEPLOYED
       APPLICATION     REUSABLE          SYSTEM
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
        DESKTOP            WEB            EMBEDDED
```

The final product does not need to resemble the original prototype. The incubator's job is to discover whether the underlying idea deserves to exist and determine the best implementation shape.