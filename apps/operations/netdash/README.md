# OPS-002 — NetDash

**Status:** seed  
**Purpose:** Inspect network interfaces, addresses, routes, traffic, and connectivity from one terminal screen.

## MVP

- Interface list
- Link state and addresses
- RX/TX counters and rates
- Default route
- Simple connectivity checks
- Refresh asynchronously

## UX

```text
INTERFACES

> eth0    UP    192.168.1.20/24    ↓42MB/s ↑8MB/s
  wlan0   DOWN
  tailscale0 UP 100.x.x.x

ROUTE
0.0.0.0/0 -> 192.168.1.1 dev eth0
```

## Experiments

Determine a portable normalized network model and compare polling cost against OS event notifications.

## Stretch

DNS diagnostics, latency history, route visualization, remote hosts, and packet-counter analytics.
