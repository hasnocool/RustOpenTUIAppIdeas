# OPS-003 — ServiceBoard

**Status:** seed  
**Purpose:** A service-health board for system services and user-defined checks.

## MVP

- Service inventory
- Running/failed/inactive states
- Start/stop/restart actions with confirmation
- Recent failure information
- Periodic health refresh

## UX

```text
SERVICES

> docker        ● running
  tailscaled     ● running
  jellyfin       ● running
  example        ✗ failed

Enter=details  r=restart  /=filter  h=history
```

## Architecture

Platform service APIs are accessed by background workers. Actions produce explicit command results and never freeze the UI.

## Stretch

Custom HTTP/TCP checks, dependency graphs, notifications, remote hosts, and historical uptime.
