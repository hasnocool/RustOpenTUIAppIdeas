# Shared Incubator Concepts

Reusable concepts belong here before they are promoted into a shared Rust crate.

```text
incubator/shared/
├── architecture/     Domain and runtime patterns
├── event-model/      Input/result/message conventions
├── terminal/         Terminal capability assumptions
├── themes/            Visual language experiments
└── widgets/           Reusable UI concepts
```

## Promotion rule

A shared concept should be promoted only after at least two applications need it or an experiment demonstrates that the abstraction reduces duplication without hiding important behavior.
