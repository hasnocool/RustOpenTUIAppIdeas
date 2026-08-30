# AI-004 — ModelBench

**Status:** seed  
**Purpose:** Repeatable local model performance benchmark explorer.

## MVP

- Define a benchmark workload
- Select a model/backend
- Run warm-up and measurement passes
- Record time-to-first-output and throughput
- Persist results
- Compare runs in a table

## Result schema

Record model, backend, software version, hardware profile, quantization/format, prompt characteristics, output size, concurrency, timestamp, and measurements.

## UX

```text
MODEL BENCH

MODEL              TTFT     TOK/S    RESULT
> qwen-7b          0.42s     48.1     ✓
  llama-8b         0.51s     44.7     ✓

[r]un [c]ompare [e]xport [d]etails
```

## Methodology

Separate cold-start and warm benchmarks. Keep workload definitions versioned so results remain comparable.

## Stretch

Regression detection, hardware profiles, performance-per-watt, performance-per-dollar, charts, and automated benchmark suites.

## Graduation

Promote when results are reproducible enough to guide actual model/backend selection.
