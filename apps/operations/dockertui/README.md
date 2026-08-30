# OPS-001 — DockerTUI

**Status:** seed  
**Purpose:** Keyboard-first container and service dashboard for local Docker environments.

## MVP

- List containers
- Running/stopped state
- CPU/memory summaries
- Start/stop/restart
- Logs viewer
- Inspect details

## UX

```text
CONTAINERS

> jellyfin        running   2.1%   1.4G
  sonarr          running   0.4%   280M
  radarr          stopped   -      -
  nginx           running   0.2%   180M

Enter=details  l=logs  r=restart  s=stop  /=filter
```

## Architecture

Docker CLI/API interactions happen in background tasks. Stream logs through bounded channels and cancel subscriptions when panes close.

## Stretch

Compose project views, image management, network/volume views, health checks, remote contexts, and event streams.

## Graduation

A safe, fast container cockpit with useful Compose awareness and strong error handling.
