# Experiment Methodology

Experiments are small, time-boxed investigations designed to answer a question.

## Experiment record

Use this structure:

```markdown
# Experiment: <name>

## Question

What uncertainty are we testing?

## Hypothesis

What do we expect?

## Setup

Terminal, OS, renderer, Rust version, data source.

## Procedure

Minimal reproducible steps.

## Observations

What actually happened?

## Measurements

Startup, render latency, memory, CPU, I/O, network, etc.

## Decision

Adopt / revise / abandon.

## Follow-up

What should happen next?
```

## Prefer evidence

For UI experiments, capture:

- keyboard path
- terminal dimensions
- screenshots where useful
- failure states
- resize behavior
- long-list behavior
- Unicode behavior
- cancellation behavior

For performance experiments, measure rather than infer.

## Reproducibility

Record OS, terminal emulator, Rust toolchain, relevant crate versions, and hardware when those variables could affect results.
