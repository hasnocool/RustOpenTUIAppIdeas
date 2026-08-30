# Incubator Architecture

The repository has two layers: the **catalog layer** and the **experiment layer**.

## Catalog layer

```text
IDEAS.md
   │
   ├── category
   │      └── app directory
   │             ├── README.md
   │             ├── SPEC.md
   │             └── ROADMAP.md
   │
   └── cross-cutting experiments
```

`IDEAS.md` answers **what exists**. An app README answers **why it exists**. `SPEC.md` answers **what it should do**. `ROADMAP.md` answers **what happens next**.

## Application architecture

When an idea becomes code, prefer a structure similar to:

```text
src/
├── main.rs              # startup and terminal lifecycle
├── app.rs               # application state/orchestration
├── event.rs             # input and internal events
├── command.rs           # user commands/keybindings
├── state.rs             # domain/UI state
├── ui/
│   ├── mod.rs
│   ├── layout.rs
│   ├── widgets.rs
│   └── theme.rs
├── domain/              # domain logic independent of rendering
├── services/            # async/background integrations
└── persistence/         # local state/config/cache
```

Do not force this exact structure on tiny experiments. The purpose is to prevent domain logic from becoming inseparable from rendering as the prototype grows.

## Event model

A preferred model is:

```text
Terminal input ───────┐
                      ▼
                 Event channel
                      │
                      ▼
                State transition
                      │
       ┌──────────────┴──────────────┐
       ▼                             ▼
 Background workers             UI renderer
       │                             │
       └──── result channel ─────────┘
```

UI code should consume already-produced results rather than synchronously performing slow work during rendering.

## Renderer boundary

Keep rendering behind a small interface where practical. This allows experiments to compare OpenTUI-compatible approaches, a Rust-native port, or another renderer without rewriting the domain model.

## Persistence

Prefer boring, inspectable formats for prototypes: TOML, JSON, SQLite, or line-oriented files depending on the problem. A prototype should not require a complex database unless persistence is itself the experiment.

## Shared components

Only extract a shared component after repeated evidence. Two applications independently implementing a command palette is a stronger signal than a speculative framework created for ten future apps.
