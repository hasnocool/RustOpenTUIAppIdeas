# Incubator TUI

The Incubator TUI is the eventual control center for this repository. It will browse `IDEAS.md`, inspect application documentation, launch experiments, track scores/status, and provide a visual map of the project.

## Concept

```text
┌───────────────────────────────────────────────────────────────┐
│ RUST OPENTUI IDEA INCUBATOR                                  │
├───────────────┬───────────────────────────────────────────────┤
│ NAVIGATION    │ IDEA LIST                                     │
│               │                                               │
│ > Dashboard   │ > AI-001  AI Router          EXPLORE         │
│   Ideas       │   AI-002  LLM TUI            BACKLOG         │
│   Research    │   DATA-001 HardwareBench     EXPLORE         │
│   Experiments │   SYS-001  SysPeek            PROTOTYPE       │
│   Prototypes  │                                               │
│   Graduated   │                                               │
├───────────────┴───────────────────────────────────────────────┤
│ DETAIL                                                        │
│ Problem / value / score / dependencies / next action         │
├───────────────────────────────────────────────────────────────┤
│ ↑↓ Navigate  Enter Open  / Search  S Score  E Experiment      │
│ P Prototype  R Research  G Graduate  ? Help  Q Quit           │
└───────────────────────────────────────────────────────────────┘
```

## Planned capabilities

- Browse all ideas.
- Filter by category, status, score, tags, and maturity.
- Full-text search across idea documentation.
- Show ASCII UX diagrams.
- Open related research and experiments.
- Create a new idea record.
- Score/re-score ideas.
- Promote ideas into experiments.
- Track experiment outcomes.
- Mark prototype/graduation decisions.
- Launch selected app prototypes.
- Display repository health and documentation coverage.

## Architecture direction

The incubator TUI should consume repository metadata through a domain model rather than directly coupling rendering code to Markdown parsing. Parsing, indexing, and filesystem operations belong in background tasks; the renderer should receive immutable snapshots or messages.

## Future source layout

```text
apps/incubator/
├── README.md
├── docs/
│   ├── UX.md
│   ├── ARCHITECTURE.md
│   ├── STATE.md
│   └── COMMANDS.md
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── state.rs
│   ├── events.rs
│   ├── commands.rs
│   ├── repository.rs
│   ├── index.rs
│   └── render.rs
└── tests/
```
