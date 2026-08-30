# Cross-cutting Experiments

This directory holds experiments that apply to multiple app ideas and do not yet deserve a shared library.

## Suggested experiment families

### Rendering

- Unicode/grapheme-aware measurement
- Responsive layouts
- Virtualized tables
- Animated progress/sparklines
- Overlay/modal composition
- Theme switching

### Interaction

- Command palette
- Fuzzy search
- Focus management
- Keymap abstraction
- Mouse hit testing
- Confirmation dialogs

### Runtime

- Async event bus
- Worker supervision
- Cancellation
- Backpressure
- Subprocess streaming
- Graceful terminal restoration

### Testing

- Render snapshots
- State-transition tests
- Input-sequence tests
- Terminal-size matrix
- Unicode fixture corpus

## Promotion rule

If an experiment is independently reusable and has been validated by at least two applications, consider extracting it into a shared crate. Until then, keeping it here prevents premature framework design.
