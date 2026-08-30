# RustOpenTUIAppIdeas

An idea incubator and exploration laboratory for small, useful, experimental terminal applications built around Rust and OpenTUI-compatible rendering approaches.

OpenTUI's upstream project currently provides a native Zig core with TypeScript bindings. This repository therefore documents Rust application ideas independently of a fixed Rust OpenTUI API; renderer/backend choices are recorded per experiment.

## Purpose

This is intentionally **not one giant application**. It is a laboratory where ideas can be captured, compared, researched, prototyped, abandoned, revived, and eventually promoted into standalone projects.

The central catalog is [`IDEAS.md`](IDEAS.md). Every application has its own directory under `apps/` with a focused README covering the problem, UX, scope, architecture, milestones, and expansion opportunities.

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
│   ├── OPENTUI_NOTES.md
│   ├── EXPERIMENTS.md
│   ├── QUALITY.md
│   └── templates/APP_README.md
├── apps/
│   ├── system/
│   ├── development/
│   ├── ai/
│   ├── data/
│   ├── operations/
│   ├── offgrid/
│   ├── productivity/
│   └── fun/
└── experiments/
```

## Principles

1. Small before large.
2. Terminal-first UX.
3. Async, non-blocking I/O by default.
4. Measure before optimizing.
5. Failed experiments are documented rather than discarded.
6. Proven ideas can graduate into standalone repositories.
7. The incubator stores concepts, prototypes, experiments, and documentation—not unrelated production infrastructure.

## Workflow

1. Browse `IDEAS.md`.
2. Open the relevant app directory.
3. Define the smallest useful experiment.
4. Research unknowns.
5. Prototype a vertical slice.
6. Record results and decisions.
7. Promote, continue, archive, or kill the idea explicitly.

See `docs/IDEA_LIFECYCLE.md` and `CONTRIBUTING.md`.
