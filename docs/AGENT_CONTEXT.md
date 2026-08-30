# Agent Context and Freshness Protocol

## Purpose

Keep agents synchronized with the incubator's current state without relying on stale conversational memory.

## Context refresh sequence

```text
┌──────────────┐
│ START TASK   │
└──────┬───────┘
       ▼
┌──────────────────────┐
│ Read AGENTS.md       │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│ Read IDEAS.md        │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│ Read DIAGRAMS.md     │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│ Inspect target app   │
│ + nearby shared docs │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│ Search for existing  │
│ related work         │
└──────────┬───────────┘
           ▼
      IMPLEMENT
           │
           ▼
┌──────────────────────┐
│ Update code + docs   │
│ + diagrams + status  │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│ Run validators/tests  │
└──────────┬───────────┘
           ▼
         DONE
```

## Canonical context files

| File | Authority |
|---|---|
| `AGENTS.md` | Operating rules |
| `IDEAS.md` | Idea inventory and IDs |
| `docs/ARCHITECTURE.md` | System architecture |
| `docs/DIAGRAMS.md` | Diagram conventions and atlas |
| `docs/IDEA_LIFECYCLE.md` | Status transitions |
| `docs/TAXONOMY.md` | Categories/tags |
| `docs/OPENTUI_NOTES.md` | Renderer research |
| `docs/QUALITY.md` | Quality gates |
| `apps/**/README.md` | App-level design |
| `incubator/**` | Experiment/prototype evidence |

## Freshness rules

- Never rely on a remembered repository tree when the repository can be inspected.
- Before adding a concept, search for equivalent or adjacent ideas.
- Before changing architecture, inspect current architecture documentation and implementation.
- When implementation contradicts documentation, treat the implementation as evidence and reconcile the documentation rather than hiding the discrepancy.
- Add a dated decision/experiment note when an important choice cannot be obvious from code.
- Keep stable IDs stable.
- Prefer links between related ideas instead of duplicating content.

## Context packet for app work

An agent should be able to reconstruct the current state from this packet:

```text
IDEA ID
  ↓
Catalog entry in IDEAS.md
  ↓
App README
  ├── UX diagrams
  ├── state/navigation
  ├── architecture
  ├── MVP
  └── milestones
  ↓
Experiments / research
  ↓
Prototype
  ↓
Implementation
  ↓
Validation
  ↓
Updated catalog + docs
```

## Handoff requirement

When stopping work, leave enough evidence for another agent to continue:

- what changed
- what was validated
- what remains
- important assumptions
- open questions
- files that are canonical
- next recommended action

Use `incubator/experiments/` or the relevant app documentation for durable handoff notes rather than depending on chat history.
