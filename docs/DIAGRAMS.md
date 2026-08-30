# Architecture & ASCII Diagram Atlas

This document is the visual source of truth for the incubator. Every major repository concept has a concrete ASCII diagram here so the project can be understood from a terminal, plain-text editor, GitHub, or printed documentation.

> Diagram policy: when architecture changes, update the corresponding ASCII diagram in the same change. Prefer diagrams that remain legible at 80 columns.

## 1. Repository at a Glance

```text
RustOpenTUIAppIdeas/
│
├── IDEAS.md                         Master idea catalog
├── README.md                        Project entry point
├── ROADMAP.md                       Incubator roadmap
├── CONTRIBUTING.md                  Contribution rules
│
├── docs/
│   ├── ARCHITECTURE.md              System architecture
│   ├── IDEA_LIFECYCLE.md            Idea progression
│   ├── TAXONOMY.md                  Categories/tags
│   ├── OPENTUI_NOTES.md             OpenTUI research
│   ├── EXPERIMENTS.md               Experiments
│   ├── QUALITY.md                   Quality gates
│   ├── DIAGRAMS.md                  This diagram atlas
│   └── templates/
│       └── APP_README.md             Per-app documentation template
│
├── incubator/
│   ├── README.md                    Incubator workspace
│   ├── ideas/                       Expanded idea records
│   ├── experiments/                 Time-boxed experiments
│   ├── prototypes/                  Working prototypes
│   ├── research/                    Research notes
│   └── shared/                      Reusable concepts
│
├── apps/
│   ├── ai/                          AI applications
│   ├── data/                        Data applications
│   ├── development/                 Developer tools
│   ├── fun/                         Experimental/fun apps
│   ├── offgrid/                     Power/off-grid apps
│   ├── operations/                  Operations/admin apps
│   ├── productivity/                Productivity apps
│   └── system/                      System utilities
│
└── scripts/
    └── validate_catalog.py          Catalog/repository validator
```

## 2. Incubator Lifecycle

```text
                  ┌─────────────────┐
                  │     CAPTURE     │
                  │ New idea        │
                  └────────┬────────┘
                           │
                           ▼
                  ┌─────────────────┐
                  │    TRIAGE       │
                  │ Deduplicate     │
                  │ Categorize      │
                  └────────┬────────┘
                           │
                           ▼
                  ┌─────────────────┐
                  │     SCORE       │
                  │ Value           │
                  │ Learning        │
                  │ Feasibility     │
                  │ Novelty         │
                  │ Reuse           │
                  │ Fun             │
                  └────────┬────────┘
                           │
                    score strong enough?
                      ┌────┴────┐
                     no        yes
                      │          │
                      ▼          ▼
                 ┌────────┐ ┌─────────────┐
                 │ BACKLOG│ │  EXPLORE    │
                 └────────┘ └──────┬──────┘
                                    │
                                    ▼
                             ┌─────────────┐
                             │ EXPERIMENT  │
                             │ Time-boxed   │
                             │ prototype    │
                             └──────┬──────┘
                                    │
                              useful?
                            ┌───────┴───────┐
                           no              yes
                            │                │
                            ▼                ▼
                       ┌────────┐    ┌─────────────┐
                       │ ARCHIVE│    │ PROTOTYPE   │
                       └────────┘    └──────┬──────┘
                                             │
                                      repeat validation
                                             │
                                             ▼
                                      ┌─────────────┐
                                      │  VALIDATED  │
                                      └──────┬──────┘
                                             │
                                  worth becoming product?
                                       ┌─────┴─────┐
                                      no          yes
                                       │            │
                                       ▼            ▼
                                  ┌────────┐  ┌───────────┐
                                  │ KEEP   │  │ GRADUATE  │
                                  │ AS IDEA│  │ Standalone│
                                  └────────┘  │ project   │
                                              └───────────┘
```

## 3. Idea → App Relationship

```text
                         IDEAS.md
                            │
                            │ idea ID
                            ▼
                    ┌───────────────┐
                    │ Idea Record   │
                    │---------------│
                    │ ID            │
                    │ title         │
                    │ category      │
                    │ status        │
                    │ score         │
                    │ app path      │
                    └───────┬───────┘
                            │
                 ┌──────────┼───────────┐
                 │          │           │
                 ▼          ▼           ▼
             research   experiment   prototype
                 │          │           │
                 └──────────┼───────────┘
                            ▼
                    apps/<category>/<app>
                            │
               ┌────────────┼─────────────┐
               ▼            ▼             ▼
            README.md    docs/        src/ (future)
```

## 4. Application Documentation Contract

```text
apps/<category>/<app>/
│
├── README.md
│   ├── Problem
│   ├── Why TUI?
│   ├── Goals
│   ├── Non-goals
│   ├── UX sketch
│   ├── MVP
│   ├── Architecture
│   ├── Async model
│   ├── Risks
│   ├── Experiments
│   └── Graduation criteria
│
├── docs/                            Optional deep documentation
│   ├── UX.md
│   ├── ARCHITECTURE.md
│   ├── STATE.md
│   ├── INPUT.md
│   └── ROADMAP.md
│
├── src/                             Added when implementation begins
│   ├── main.rs
│   ├── app.rs
│   ├── state.rs
│   ├── events.rs
│   ├── commands.rs
│   └── render.rs
│
└── tests/                           Added when behavior stabilizes
    ├── state_tests.rs
    └── snapshots/
```

## 5. Recommended Runtime Architecture

```text
                    ┌──────────────────────┐
                    │       TERMINAL       │
                    └──────────┬───────────┘
                               │ input
                               ▼
                    ┌──────────────────────┐
                    │    INPUT ADAPTER     │
                    │ crossterm / backend  │
                    └──────────┬───────────┘
                               │
                               ▼
                    ┌──────────────────────┐
                    │     EVENT QUEUE      │
                    └──────────┬───────────┘
                               │
                 ┌─────────────┴─────────────┐
                 │                           │
                 ▼                           ▼
        ┌─────────────────┐        ┌──────────────────┐
        │   UI / STATE    │        │ BACKGROUND TASKS │
        │ single owner    │◄───────│ async workers    │
        └────────┬────────┘ result└─────────┬────────┘
                 │                           │
                 │ render commands          │
                 ▼                           ▼
        ┌─────────────────┐          filesystem/network
        │ RENDERER        │
        │ OpenTUI boundary│
        └────────┬────────┘
                 │
                 ▼
             TERMINAL
```

The UI/rendering owner should not perform blocking filesystem, process, network, or database work. Background work communicates through bounded channels/events.

## 6. OpenTUI Boundary

```text
┌───────────────────────────────────────────────────────────┐
│                     APPLICATION                           │
│                                                           │
│  state ──► commands ──► domain services ──► results      │
│    │                                                │      │
│    └──────────────────────┬─────────────────────────┘      │
│                           ▼                                │
│                    render model                            │
└───────────────────────────┬───────────────────────────────┘
                            │
                            ▼
                 ┌──────────────────────┐
                 │   TUI ADAPTER LAYER  │
                 │ layout / widgets /   │
                 │ input translation    │
                 └──────────┬───────────┘
                            │
                            ▼
                 ┌──────────────────────┐
                 │     OPENTUI CORE     │
                 │ buffer / cells /     │
                 │ clipping / rendering │
                 └──────────┬───────────┘
                            │
                            ▼
                        TERMINAL
```

OpenTUI is treated as a rendering boundary, not as the application's domain model.

## 7. Incubator Meta-TUI

```text
┌─────────────────────────────────────────────────────────────────────┐
│ RUST OPENTUI IDEA INCUBATOR                              v0.1       │
├───────────────────────┬─────────────────────────────────────────────┤
│ CATEGORIES            │ IDEAS                                       │
│                       │                                             │
│ > AI              4  │ > AI Router          30/30   EXPLORE       │
│   Data             3  │   LLM TUI             29/30   EXPLORE       │
│   Development     4  │   HardwareBench       28/30   BACKLOG       │
│   Fun              2  │   GitDash             28/30   EXPLORE       │
│   OffGrid          2  │   SolarTUI            27/30   BACKLOG       │
│   Operations       3  │   DockerTUI           27/30   BACKLOG       │
│   Productivity     3  │                                             │
│   System           5  │                                             │
├───────────────────────┴─────────────────────────────────────────────┤
│ SELECTED IDEA                                                      │
│ AI-001  AI Router                                                   │
│                                                                     │
│ Problem: choose the best available model for each task.            │
│                                                                     │
│ [M] MVP  [E] Experiment  [R] Research  [D] Documentation           │
│ [S] Score  [T] Tags       [P] Prototype  [G] Graduate              │
├─────────────────────────────────────────────────────────────────────┤
│ ↑↓ Navigate  Enter Open  / Search  Tab Panel  ? Help  Q Quit       │
└─────────────────────────────────────────────────────────────────────┘
```

## 8. Meta-TUI Navigation

```text
                         ┌───────────────┐
                         │   DASHBOARD   │
                         └───────┬───────┘
                                 │
       ┌─────────────────────────┼─────────────────────────┐
       ▼                         ▼                         ▼
  ┌──────────┐             ┌──────────┐             ┌───────────┐
  │  IDEAS   │             │ RESEARCH │             │ EXPERIMENT│
  └────┬─────┘             └────┬─────┘             └─────┬─────┘
       │                        │                           │
       ▼                        ▼                           ▼
  ┌──────────┐             ┌──────────┐             ┌───────────┐
  │ DETAILS  │             │ NOTES    │             │ RESULTS   │
  └────┬─────┘             └──────────┘             └─────┬─────┘
       │                                                   │
       └───────────────────────┬───────────────────────────┘
                               ▼
                        ┌──────────────┐
                        │  PROTOTYPE   │
                        └──────┬───────┘
                               │
                         validation
                               │
                               ▼
                        ┌──────────────┐
                        │   GRADUATE   │
                        └──────────────┘
```

## 9. Event / Async Flow

```text
Terminal event
     │
     ▼
┌───────────────┐
│ event reader  │
│ dedicated task│
└──────┬────────┘
       │ Event
       ▼
┌───────────────┐       command        ┌────────────────┐
│ bounded input │─────────────────────►│ application    │
│ channel       │                      │ state owner    │
└───────────────┘                      └───────┬────────┘
                                               │
                              spawn/send work  │
                                               ▼
                                      ┌────────────────┐
                                      │ worker task    │
                                      │ async I/O      │
                                      └───────┬────────┘
                                              │ result
                                              ▼
                                      ┌────────────────┐
                                      │ bounded result │
                                      │ channel        │
                                      └───────┬────────┘
                                              │
                                              ▼
                                      state update
                                              │
                                              ▼
                                         render
```

## 10. Shared Crate Evolution

```text
                         shared concepts
                               │
       ┌───────────────────────┼────────────────────────┐
       ▼                       ▼                        ▼
┌─────────────┐        ┌──────────────┐        ┌──────────────┐
│ terminal    │        │ event model  │        │ theme system │
└──────┬──────┘        └──────┬───────┘        └──────┬───────┘
       │                      │                       │
       └──────────────────────┼───────────────────────┘
                              ▼
                       ┌──────────────┐
                       │ shared TUI   │
                       │ primitives   │
                       └──────┬───────┘
                              │
          ┌───────────────────┼─────────────────────┐
          ▼                   ▼                     ▼
       AI apps           Ops apps              System apps
```

## 11. Idea Scoring

```text
                 IDEA SCORE / 30
                       │
       ┌───────────────┼────────────────┐
       ▼               ▼                ▼
   USER VALUE      LEARNING VALUE    FEASIBILITY
      /5                /5               /5
       │               │                │
       └───────────────┼────────────────┘
                       │
       ┌───────────────┼────────────────┐
       ▼               ▼                ▼
     NOVELTY         REUSE              FUN
       /5               /5               /5
                       │
                       ▼
                 TOTAL / 30
```

## 12. Graduation Decision

```text
                    prototype works
                          │
                          ▼
                   ┌─────────────┐
                   │ Useful?     │
                   └──────┬──────┘
                      no  │  yes
                 ┌────────┘  └────────┐
                 ▼                     ▼
             archive             ┌────────────┐
                                  │ Repeatable?│
                                  └─────┬──────┘
                                    no  │  yes
                               ┌────────┘  └───────┐
                               ▼                   ▼
                          incubator          ┌────────────┐
                                             │ Maintainable│
                                             └──────┬─────┘
                                                no  │ yes
                                           ┌────────┘  └──────┐
                                           ▼                  ▼
                                       prototype          GRADUATE
```

## 13. Category Map

```text
                         APPLICATION IDEAS
                                │
       ┌──────────┬─────────────┼──────────────┬──────────────┐
       ▼          ▼             ▼              ▼              ▼
      AI         DATA      DEVELOPMENT      SYSTEM        OPERATIONS
       │          │             │              │              │
       ▼          ▼             ▼              ▼              ▼
    models     datasets       Git/docs       processes      services
    routing    explorers      repos          disks          Docker
    chat       benchmarks     config         logs           network

       ┌──────────┬─────────────┬──────────────┐
       ▼          ▼             ▼              ▼
    OFFGRID   PRODUCTIVITY     FUN          FUTURE
       │          │             │              │
       ▼          ▼             ▼              ▼
     solar       tasks        visual       new categories
     power       notes        experiments  discovered ideas
     forecast   timers        toys
```

## 14. Prototype Evolution

```text
README concept
     │
     ▼
static ASCII mockup
     │
     ▼
state-only prototype
     │
     ▼
renderer prototype
     │
     ▼
interactive TUI
     │
     ▼
real data adapter
     │
     ▼
async background work
     │
     ▼
tests + snapshots
     │
     ▼
usable application
```

## 15. Quality Gates

```text
Idea
 │
 ├── documentation complete? ──no──► improve docs
 │
 yes
 │
 ├── ASCII UX mockup? ─────────no──► add mockup
 │
 yes
 │
 ├── async model documented? ──no──► document I/O
 │
 yes
 │
 ├── MVP bounded? ─────────────no──► reduce scope
 │
 yes
 │
 ├── prototype tested? ────────no──► experiment
 │
 yes
 │
 └──────────────────────────────────► graduation review
```

## 16. 80-Column Diagram Rule

```text
01234567890123456789012345678901234567890123456789012345678901234567890123456789
|------------------------------------------------------------------------------|
|                         TARGET DIAGRAM WIDTH                                 |
|------------------------------------------------------------------------------|
```

Keep primary diagrams below 80 columns whenever practical. If a diagram requires more width, provide a compact 80-column version first.
