# Contributing to the Incubator

The goal is to maximize useful experimentation while keeping the repository understandable as the idea count grows.

## Adding an idea

1. Choose a category under `apps/`.
2. Reserve a stable ID in `IDEAS.md`.
3. Create a kebab-case directory.
4. Copy `docs/templates/APP_README.md`.
5. Describe the smallest useful vertical slice.
6. Record dependencies and technical unknowns.
7. Add the idea to the master catalog.

## Idea quality standard

A good idea description answers:

- What problem does this solve?
- Who would use it?
- Why is a TUI appropriate?
- What is the smallest useful version?
- What can be deliberately excluded from v0.1?
- What is technically uncertain?
- What can be reused elsewhere?
- What would make the idea worth graduating?

## Async rule

Rust applications in this incubator should avoid blocking operations on the UI/event loop. File access, subprocesses, network requests, timers, and long-running computations should run in appropriate asynchronous/background tasks or dedicated worker threads, communicating results through channels. Do not use a blocking call merely because it is convenient.

## Prototype rule

A prototype should prove one important thing, not attempt the entire product. Prefer:

```text
input -> state transition -> render -> observable result
```

over a large architecture with no validated behavior.

## Documentation rule

Every app directory should contain at least:

- `README.md` — concept and scope
- `SPEC.md` — behavior and UX contract once the idea becomes more concrete
- `ROADMAP.md` — staged implementation plan

Seed ideas may begin with only `README.md`; add the other documents when exploration starts.

## Graduation

When an app has a stable identity, a working vertical slice, and enough implementation complexity to justify its own lifecycle, create a standalone repository and leave a short graduation note in this incubator.

## Killing an idea

Rejected ideas are valuable. Do not silently delete them. Mark the status `rejected` or `archived` and record the reason, evidence, and lessons learned.
