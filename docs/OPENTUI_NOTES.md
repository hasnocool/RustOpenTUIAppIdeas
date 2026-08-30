# OpenTUI and Rust Notes

## Current upstream reality

The upstream OpenTUI project is a native Zig terminal UI core with TypeScript bindings. It exposes a C ABI and is designed as a rendering/component foundation. It includes layout, input handling, terminal capability detection, testing support, and optimized cell updates.

This repository uses the name **RustOpenTUIAppIdeas** to describe the target ecosystem and application direction, not to assert that the upstream project is itself a Rust crate.

## Rust strategy

Applications should keep their domain logic independent from the renderer. This makes it possible to evaluate:

1. a Rust-native OpenTUI port/implementation,
2. an FFI bridge to the OpenTUI native core,
3. another Rust TUI renderer when appropriate.

The correct choice should be made per experiment and recorded in the app's documentation.

## Important UI concerns

OpenTUI's documentation emphasizes the distinction between UTF-8 bytes, Unicode code points, grapheme clusters, and terminal display cells. Rust experiments should make the same distinction and avoid naïve string-length calculations for layout.

Terminal capabilities also vary. Features such as hyperlinks, images, clipboard behavior, mouse input, and pixel protocols should be treated as optional capabilities rather than assumptions.

## Suggested Rust building blocks

Potential building blocks include:

- `crossterm` for terminal I/O/input where appropriate
- `tokio` for asynchronous application work
- `unicode-segmentation` for grapheme boundaries
- `unicode-width` for terminal width calculations
- `ropey` for serious text editing experiments
- `serde` + `serde_json`/`toml` for persistence/configuration
- `criterion` for benchmarks

Crate versions should be checked at implementation time rather than hard-coded into idea documents.

## Renderer-neutral contract

Where practical, define application state and commands as:

```text
InputEvent -> Command -> State -> RenderModel -> Renderer
```

This keeps business logic testable without a real terminal.

## Non-blocking requirement

Do not perform blocking filesystem, subprocess, network, or long-running computation on the UI loop. Use asynchronous tasks or worker threads and communicate through channels. Rendering should consume ready state and remain responsive.

## References

- Upstream OpenTUI documentation: https://opentui.com/docs/
- Upstream project: https://github.com/anomalyco/opentui
- Rust OpenTUI port research candidate: https://github.com/Dicklesworthstone/opentui_rust
