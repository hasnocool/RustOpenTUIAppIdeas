# PROD-001 — TaskForge

**Status:** seed  
**Purpose:** Fast keyboard-first task management with minimal ceremony.

## MVP

- Inbox
- Tasks with status/priority
- Tags
- Due dates
- Search
- Complete/archive
- Local persistence

## UX

```text
TASKFORGE

INBOX (5)
> Fix renderer resize bug       high
  Research model metadata       med
  Update documentation          low
  Test CSV import               med

Enter=edit  x=complete  a=add  /=search
```

## Architecture

Keep task/domain state separate from UI state. Persistence should be asynchronous where file size or storage backend could make writes slow; use atomic saves and explicit error states.

## Stretch

Recurring tasks, projects, dependencies, time tracking, Markdown notes, and synchronization.

## Graduation

Only if the workflow becomes a genuinely preferred task system rather than another TODO clone.
