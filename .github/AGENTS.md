# GitHub Automation Agent Rules

- Keep CI focused on catalog integrity, diagram integrity, formatting, tests, and build correctness.
- Never weaken a validation rule just to make a workflow pass.
- Changes to idea status should update `IDEAS.md` and the app README together.
- Changes to diagrams should run the ASCII validator.
- Prefer deterministic scripts with no network dependency for repository validation.
- Do not auto-delete ideas, experiments, or research. Archive explicitly.
- Keep generated reports separate from canonical source documents.
