# PROD-003 — Pomodoro TUI

**Status:** seed  
**Purpose:** A minimal timer and work-session tracker that lives comfortably beside a terminal workflow.

## MVP

- Configurable work/break durations
- Start/pause/reset
- Session counter
- Current task label
- Desktop/terminal notification when supported
- Persist daily session totals

## UX

```text
POMODORO

             24:37

        Research TUI ideas

████████████████░░░░  82%

[s]tart [p]ause [r]eset [n]ext [q]uit
```

## Architecture

Use monotonic time rather than counting render frames. Timer state should remain correct even if the UI is temporarily delayed.

## Stretch

Task integration, daily statistics, configurable schedules, sound/notification adapters, and session history.
