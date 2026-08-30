# AGENTS.md — RustOpenTUIAppIdeas Agent Operating Contract

## Mission

This repository is an idea incubator and exploration laboratory for Rust terminal applications and OpenTUI-compatible rendering approaches. Agents must preserve the repository's role as a place to capture, research, visualize, prototype, evaluate, and graduate ideas.

## Source-of-truth hierarchy

1. `AGENTS.md` and `docs/AGENT_CONTEXT.md` — agent operating rules.
2. `IDEAS.md` — canonical idea catalog and stable idea IDs.
3. `docs/` — shared architecture, lifecycle, taxonomy, quality, diagrams, and renderer notes.
4. `apps/<category>/<app>/README.md` — canonical application concept documentation.
5. `incubator/` — experiments, research, prototypes, and shared incubator assets.
6. Source code — implementation reality; update documentation when reality changes.

## Mandatory behavior

- Read `AGENTS.md`, `docs/AGENT_CONTEXT.md`, `docs/DIAGRAMS.md`, and the relevant app README before modifying an app.
- Search the repository before creating a new idea, component, diagram, or shared abstraction.
- Preserve stable idea IDs and existing directory mappings unless deliberately migrating them.
- Every new app idea must be represented in `IDEAS.md` and have an app directory plus README.
- Every meaningful UI or architecture proposal must include ASCII diagrams where spatial relationships, navigation, state, or data flow matter.
- Keep diagrams ASCII-only and suitable for an 80-column terminal unless a document explicitly states a wider format.
- Keep diagrams synchronized with implementation. Stale diagrams are defects.
- Prefer small vertical slices over speculative frameworks.
- Keep application/domain logic independent from the renderer/backend boundary.
- Async work must be non-blocking. Never put blocking filesystem, subprocess, network, or expensive computation on the UI/event loop.
- Use bounded channels and cancellation where background work can accumulate or outlive a screen.
- Record assumptions and unresolved questions rather than silently inventing APIs.
- When upstream OpenTUI/Rust bindings change, update `docs/OPENTUI_NOTES.md` and affected experiments.
- Run repository validators and formatting/checks appropriate to changed code.

## Documentation contract for every app

Each `apps/<category>/<app>/README.md` should contain, when applicable:

- status and idea ID
- problem statement
- goals / non-goals
- users and workflows
- ASCII UX mockup
- ASCII navigation/state diagram
- ASCII architecture/data-flow diagram
- MVP
- implementation direction
- async/I/O strategy
- risks and unknowns
- experiments
- milestones
- graduation criteria

## Agent context freshness

Before implementation, establish:

```text
CURRENT REPO
    │
    ├── catalog → IDEAS.md
    ├── architecture → docs/
    ├── idea → app README
    ├── prototype → incubator/prototypes/
    └── implementation → source/tests
          │
          ▼
     reconcile docs
```

After implementation, update the affected documentation, diagrams, status, decisions, and experiment notes in the same change whenever practical.

## Do not

- Do not replace existing documentation wholesale when a focused update is sufficient.
- Do not remove diagrams merely to shorten files.
- Do not claim an app is implemented when only its concept/prototype exists.
- Do not couple every idea to an unstable renderer API.
- Do not introduce blocking operations into async paths.
- Do not create duplicate ideas under different names without documenting the relationship.

## Completion standard

A change is complete when code, tests/validation, documentation, diagrams, catalog metadata, and status all agree with one another.
