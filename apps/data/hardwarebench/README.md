# DATA-003 — HardwareBench

**Status:** exploring  
**Purpose:** Browse and compare hardware measurements relevant to local computing and terminal workloads.

## MVP

- Hardware inventory schema
- Model/CPU/GPU/RAM/storage fields
- Benchmark result records
- Filter and sort
- Compare selected entries
- Import/export structured data

## UX

```text
HARDWARE BENCH

DEVICE             RAM     GPU/ACCEL     SCORE    POWER
> Ryzen Mini PC    32GB    Radeon iGPU  8120     45W
  Apple Silicon    24GB    integrated   9100     25W
  Desktop          64GB    discrete     18200    220W
```

## Architecture

Separate normalized hardware records from benchmark observations. A benchmark result should include workload, software/backend, version, date, and measurement conditions.

## Stretch

Price history, performance-per-watt, performance-per-dollar, model-fit estimates, vendor metadata, and research URL references.

## Graduation

Promote when the dataset and benchmark methodology are reliable enough to support actual hardware-selection decisions.
