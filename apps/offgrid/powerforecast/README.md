# OFF-002 — PowerForecast

**Status:** seed  
**Purpose:** Estimate future energy production, consumption, and battery trajectory from historical telemetry and forecast inputs.

## MVP

- Import historical power samples
- Compute hourly/daily aggregates
- Estimate near-term PV production
- Estimate load trend
- Display projected battery trajectory
- Show uncertainty clearly

## UX

```text
POWER FORECAST

NOW       780 W PV     310 W load
14:00     1.42 kW PV   340 W load
16:00     1.05 kW PV   320 W load
18:00     0.31 kW PV   290 W load
20:00     0.00 kW PV   250 W load

Battery projection
████████████████░░░░  81% -> 64%
```

## Architecture

Forecast calculation is a background job. Keep raw observations separate from derived forecasts so models can be replaced without changing the UI.

## Stretch

Weather-aware forecasting, configurable load scenarios, model comparison, alerts, and recommendation experiments.

## Graduation

Only after forecasts prove useful against recorded data; accuracy should be measured rather than assumed.
