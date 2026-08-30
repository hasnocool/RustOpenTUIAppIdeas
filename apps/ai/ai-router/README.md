# AI-003 — AI Router

**Status:** seed  
**Purpose:** Select an appropriate AI backend for a task using capability, latency, availability, context size, and cost metadata.

## MVP

- Define backend registry
- Define model capability metadata
- Submit a task
- Score candidate models
- Route to the selected backend
- Display why it was selected
- Fall back on recoverable failures

## Decision model

```text
Task
 │
 ├─ required context
 ├─ capability
 ├─ latency target
 ├─ budget
 └─ availability
        │
        ▼
   Candidate scorer
        │
        ▼
    Selected model
        │
        ▼
   Async execution
```

## Key experiment

Start with deterministic rules before introducing learned routing. Every routing decision should be explainable.

## Stretch

Historical performance, task classification, benchmark-informed routing, queue management, circuit breakers, and provider health.

## Risks

Routing can become more complicated than the workload. Keep the first version small and measurable.

## Graduation

A standalone router is justified if it consistently improves latency, cost, or reliability over manual model selection.
