# Idea Lifecycle

The incubator treats ideas as evolving artifacts rather than permanent commitments.

## 1. Seed

Capture the idea in one paragraph. Define the user/problem and why a TUI could be better than a GUI/web UI for the workflow.

Output:

- catalog entry
- README
- initial score

## 2. Explore

Identify assumptions and research questions.

Typical questions:

- Is the required data accessible locally?
- Does a suitable Rust crate exist?
- Is the terminal a good interaction surface?
- What is the minimum useful data model?
- What must remain responsive?
- What terminal capabilities are required?

Output:

- research notes
- risk list
- proposed vertical slice

## 3. Prototype

Build one narrow end-to-end path. Avoid polishing secondary features before the primary workflow works.

A successful prototype should demonstrate:

```text
real input → real state → real rendering → real user action
```

## 4. Validate

Test the idea against actual use. Record friction, performance, terminal compatibility, and whether the interaction model is better than alternatives.

## 5. Incubate

If the concept is useful but not yet a standalone product, expand it here. Add `SPEC.md`, `ROADMAP.md`, benchmarks, screenshots, and design experiments as needed.

## 6. Graduate

Move mature projects into their own repository when they have:

- a stable name and scope
- a meaningful vertical slice
- documented architecture
- repeatable build/test instructions
- a reason for an independent release lifecycle

## 7. Pause / archive / reject

These are healthy outcomes. Record why the idea stopped and what evidence would justify revisiting it.

## Decision record

Every major transition should answer:

> What did we learn that changed our confidence in this idea?

That question keeps the incubator evidence-driven rather than becoming a collection of speculative feature lists.
