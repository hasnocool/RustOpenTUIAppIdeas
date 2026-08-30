# SYS-005 — LogView

**Status:** seed  
**Purpose:** A responsive terminal log viewer for files and process/service streams.

## MVP

- Open a log file
- Follow appended lines
- Pause/resume following
- Search and filter
- Jump to latest
- Highlight severity tokens
- Handle rotation/truncation

## UX

```text
LOGVIEW  /var/log/example.log   FOLLOW ●
────────────────────────────────────────────
12:41:03 INFO   server started
12:41:04 INFO   connection accepted
12:41:07 WARN   retrying request
>12:41:08 ERROR  request failed
────────────────────────────────────────────
/follow  n=next  N=previous  g=top  G=bottom
```

## Async design

A file watcher/reader runs outside the UI loop and emits bounded batches. The UI should apply backpressure and cap retained history.

## Stretch

Multiple panes, JSON logs, regex filters, journal/system-service sources, saved searches, and remote streams.

## Graduation

Worth promoting if it becomes a reliable daily troubleshooting tool with fast search on very large logs.
