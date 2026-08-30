# TUI Design Skill

## Trigger
Use when designing or changing terminal UX, navigation, layout, state, keyboard controls, or rendering behavior.

## Procedure

```text
USER GOAL
   ↓
SCREEN MAP
   ↓
ASCII MOCKUP
   ↓
NAVIGATION / STATE DIAGRAM
   ↓
DATA / EVENT FLOW
   ↓
COMPONENT BOUNDARIES
   ↓
IMPLEMENTATION
   ↓
TERMINAL VALIDATION
```

## Mandatory diagrams

For non-trivial screens include:

1. screen mockup
2. navigation/state map
3. event/data flow

Keep diagrams readable in an 80-column terminal. Use plain ASCII box-drawing characters (`+`, `-`, `|`, `>`, `<`, `^`, `v`) for maximum portability unless Unicode is explicitly being tested.

## Interaction rules

- Every action must have a visible or documented keybinding.
- Focus state must be unambiguous.
- Long-running operations must expose progress/status.
- Errors must not destroy the current navigation state.
- Resize behavior must be considered.
- Avoid modal depth that traps users without an obvious escape path.

## Accessibility

Do not rely exclusively on color. Use symbols, labels, selection markers, borders, and text to convey state.
