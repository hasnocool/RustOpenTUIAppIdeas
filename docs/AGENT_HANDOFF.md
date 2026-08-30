# Agent Handoff Ledger

This file is the durable handoff point for work that spans agents or sessions.

## Current state

- Agent operating contract: `AGENTS.md`
- Freshness protocol: `docs/AGENT_CONTEXT.md`
- Master catalog: `IDEAS.md`
- Diagram atlas: `docs/DIAGRAMS.md`
- Incubator TUI: `apps/incubator/`
- Experiments: `incubator/experiments/`
- Prototypes: `incubator/prototypes/`

## Active objective

Build the repository-aware incubator TUI that discovers the catalog and application documentation, renders ASCII diagrams, supports search/filtering/scoring, and provides navigation through the idea graph.

## Current implementation principle

```text
IDEAS.md + app READMEs + incubator evidence
                    |
                    v
             discovery/indexing
                    |
                    v
              application state
                    |
                    v
              TUI renderer
```

## Next work packet

1. Define a machine-readable catalog parser without making Markdown itself obsolete.
2. Discover `apps/*/*/README.md` at runtime.
3. Extract fenced ASCII/text diagrams for preview.
4. Implement screens: dashboard, ideas, app detail, diagram browser, search, experiments.
5. Add keyboard navigation and explicit help.
6. Keep filesystem reads off the UI event loop.
7. Add fixture-based tests for catalog and diagram discovery.
8. Update `docs/DIAGRAMS.md` with the real implementation flow.

## Handoff notes

Keep this ledger concise. Detailed design belongs in the canonical documentation files above.
