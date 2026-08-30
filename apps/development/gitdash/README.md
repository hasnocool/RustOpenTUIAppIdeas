# DEV-001 — GitDash

**Status:** seed  
**Purpose:** Keyboard-first Git repository cockpit for status, branches, commits, diffs, and common workflows.

## MVP

- Repository status
- Changed-file list
- Diff viewer
- Branch browser
- Recent commits
- Fetch/status refresh

## UX

```text
GITDASH  ~/src/project

BRANCH  main   ↑0 ↓0   STATUS  3 changed

> M src/app.rs
  M src/ui/layout.rs
  ? docs/notes.md

[d]iff [b]ranches [c]ommits [f]etch [/]search [q]uit
```

## Architecture

Git commands are subprocess work and must never block rendering. Prefer a command service with cancellation, bounded output, and explicit exit-status/error handling.

## Stretch

Interactive staging, commit composition, worktrees, stash browser, conflict assistance, remote/PR summaries, and repository health checks.

## Graduation

A dependable terminal Git cockpit with excellent diff navigation and safe state-changing operations.
