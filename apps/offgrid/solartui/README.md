# OFF-001 — SolarTUI

**Status:** exploring  
**Purpose:** Live terminal dashboard for solar production, battery state, loads, and historical energy data.

## MVP

- Current PV power
- Battery voltage/current/state
- Load estimate
- Daily energy counters
- Refresh interval
- Historical sparkline

## UX

```text
SOLAR                 BATTERY               LOAD
1.24 kW               13.42 V               342 W
3.81 kWh today        78% estimated         342 W

PV   ███████████████░░░░  1.24 kW
SOC  ███████████████░░░░  78%

06 08 10 12 14 16 18 20
▂  ▃  ▅  █  █  ▇  ▃  ▁
```

## Architecture

Sensor/Modbus/API readers run as cancellable background tasks. Normalize telemetry into timestamped samples; rendering consumes immutable snapshots.

## Stretch

Forecasting, weather integration, controller configuration, alerts, battery history, energy-cost estimates, and remote telemetry.

## Graduation

Promote if it becomes a reliable operational dashboard with a stable telemetry model and useful historical analysis.
