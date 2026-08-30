# Incubator Workspace

The `incubator/` tree is the working laboratory for ideas before they become fully implemented applications.

```text
incubator/
├── ideas/
│   └── <idea-id>-<slug>.md
├── experiments/
│   └── <experiment-id>-<slug>/
│       ├── README.md
│       ├── NOTES.md
│       └── RESULTS.md
├── prototypes/
│   └── <idea-id>-<slug>/
├── research/
│   └── <topic>.md
└── shared/
    ├── event-model/
    ├── terminal/
    ├── themes/
    ├── widgets/
    └── architecture/
```

## Purpose

The incubator separates **exploration** from **production implementation**. Ideas can be researched, mocked, measured, rejected, combined, or revived without creating unnecessary application code.

## Working Rules

1. Every idea has a stable ID in `IDEAS.md`.
2. Research should answer a concrete question.
3. Experiments should be time-boxed and have an explicit hypothesis.
4. ASCII mockups are first-class artifacts.
5. Prototype code should not become production code accidentally.
6. Successful prototypes graduate into `apps/`.
7. Failed experiments are valuable results and should document why they failed.
