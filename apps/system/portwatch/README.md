# SYS-004 — PortWatch

**Status:** seed  
**Purpose:** Show listening TCP/UDP ports, owning processes, addresses, and connection summaries.

## MVP

- Enumerate listeners
- Filter by port/process/address
- Sort columns
- Detail view
- Refresh asynchronously

## UX

```text
PROTO  LOCAL ADDRESS       PORT   PID     PROCESS
TCP    0.0.0.0             22     812     sshd
TCP    127.0.0.1           5432   1442    postgres
TCP    0.0.0.0             8080   2931    service
```

## Experiments

Compare platform APIs and determine the smallest normalized connection model.

## Stretch

Connection counts, service metadata, firewall context, remote-host mode, and export.
