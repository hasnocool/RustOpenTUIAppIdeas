# Context Sync Skill

## Trigger
Use before and after substantial repository changes, especially when multiple agents may work concurrently.

## Before work

```text
FETCH CURRENT BRANCH
       ↓
READ AGENT CONTRACT
       ↓
READ CATALOG
       ↓
SEARCH RELATED FILES
       ↓
CHECK CURRENT IMPLEMENTATION
       ↓
BUILD CONTEXT PACKET
```

## After work

```text
IMPLEMENT
   ↓
UPDATE DOCS
   ↓
UPDATE ASCII DIAGRAMS
   ↓
UPDATE STATUS / ROADMAP
   ↓
RUN VALIDATORS
   ↓
WRITE HANDOFF NOTE
```

## Conflict resolution

- Current repository state beats stale chat context.
- Explicit current documentation beats inferred intent.
- Tests and executable behavior are evidence of implementation state.
- Preserve user-requested scope; do not silently narrow it.
- If two docs conflict, reconcile them in the same change and record the decision.

## Handoff format

```text
## Agent Handoff
Date:
Scope:
Changed:
Validated:
Known limitations:
Open questions:
Next action:
Canonical files:
```
