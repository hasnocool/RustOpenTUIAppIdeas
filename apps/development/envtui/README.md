# DEV-003 — EnvTUI

**Status:** seed  
**Purpose:** Inspect environment variables and development configuration without dumping everything into a shell.

## MVP

- List variables
- Search/filter names
- Reveal/hide values
- Show source/context where available
- Copy selected value through an explicit user action

## Safety

Values may contain secrets. The UI should default to masking sensitive-looking variables and should never write environment values to logs.

## Stretch

Compare environments, inspect `.env` files, detect missing variables, and generate sanitized diagnostics.

## Graduation

Only if it provides a clearly safer and more useful workflow than existing shell commands.
