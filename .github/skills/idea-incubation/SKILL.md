# Idea Incubation Skill

## Trigger
Use when creating, researching, scoring, refining, prototyping, archiving, or graduating an application idea.

## Procedure

1. Read `AGENTS.md`, `docs/AGENT_CONTEXT.md`, and `IDEAS.md`.
2. Search for related ideas before creating a new one.
3. Assign or preserve a stable ID.
4. Place the idea in the appropriate taxonomy category.
5. Give it a documented status and score.
6. Create/update its app README.
7. Add ASCII UX, navigation/state, and architecture diagrams where relevant.
8. Record unknowns and experiments.
9. Validate catalog paths and diagrams.
10. Update status only when evidence supports the transition.

## Required idea record

```text
ID
Name
Category
Status
Score
Problem
Users
MVP
Key UX
Architecture
Async/I/O
Risks
Experiments
Graduation criteria
Related ideas
```

## Status model

```text
IDEA → EXPLORING → RESEARCHING → PROTOTYPING → VALIDATING
                                      │              │
                                      ├──────────────┤
                                      ▼              ▼
                                   ARCHIVED       GRADUATED
```

## Quality rule

A high score is not evidence of viability. Prototype evidence, user value, implementation feasibility, and maintenance cost must be considered separately.
