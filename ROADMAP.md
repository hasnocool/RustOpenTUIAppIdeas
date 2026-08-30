# Incubator Roadmap

## Phase 0 — Foundation

- [x] Create master catalog
- [x] Define idea lifecycle
- [x] Define taxonomy and scoring
- [x] Establish app-directory conventions
- [x] Document OpenTUI/Rust boundary
- [ ] Add automated catalog validation
- [ ] Add CI for Markdown/link consistency

## Phase 1 — Exploration infrastructure

- [ ] Build a reusable TUI application skeleton
- [ ] Define shared event/state patterns
- [ ] Define async worker/channel patterns
- [ ] Create benchmark result schema
- [ ] Create terminal capability test matrix
- [ ] Add screenshot/snapshot documentation conventions

## Phase 2 — First vertical slices

Recommended learning sequence:

1. TaskForge
2. SysPeek
3. JSON Explorer
4. GitDash
5. LogView
6. DockerTUI
7. LLM-TUI
8. HardwareBench
9. SolarTUI
10. AI Router

## Phase 3 — Reusable foundations

Extract repeated patterns into shared crates/modules only after at least two applications need them:

- navigation
- command palette
- tables
- modals
- notifications
- async job supervision
- configuration
- themes
- persistence
- metrics

## Phase 4 — Graduation

Promote the strongest applications into dedicated repositories. Each graduation should include a design retrospective describing what was learned in the incubator.

## Guiding rule

Do not turn the incubator itself into a framework prematurely. The repository exists to discover which abstractions are actually worth keeping.
