# FUN-002 — QR TUI

**Status:** seed  
**Purpose:** Generate and preview QR codes directly in the terminal as a compact rendering experiment.

## MVP

- Enter text
- Generate QR matrix
- Render using Unicode/block characters
- Adjust size/margin
- Save a textual representation

## UX

```text
QR TUI

Text: https://example.test

████████████████████
██  ██      ██  ████
██  ██████  ██  ████
... terminal QR ...
████████████████████
```

## Experiment

Focus on cell aspect ratio, terminal Unicode support, scaling, quiet zones, and narrow-terminal behavior.

## Stretch

Clipboard input, multiple rendering modes, image export, links, and a small reusable QR renderable.
