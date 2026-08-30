# Quality Gates

The incubator should stay lightweight, but every prototype should have a clear quality bar.

## Baseline

- Builds with the intended Rust toolchain.
- No obvious panic paths in normal interaction.
- Clean terminal restoration on exit, including error paths.
- UI remains responsive while background work runs.
- Keyboard navigation is discoverable.
- Resize does not corrupt state.
- Long lists can scroll without unbounded memory growth.
- Unicode text is handled with display width rather than byte length.

## Async checks

Review every I/O operation and ask:

> Could this take long enough to make the UI visibly freeze?

If yes, move it out of the UI loop. Ensure cancellation and shutdown are handled cleanly.

## Prototype performance

Do not demand premature micro-optimization. Do record obvious bottlenecks, especially:

- repeated filesystem scans
- subprocess polling
- network polling
- full-screen redraws
- expensive parsing on every frame
- unbounded logs/history

## Graduation quality

A project ready to graduate should have tests for domain logic, a reproducible build, documented configuration, basic error handling, a clear command/keymap contract, and a short performance/compatibility note.
