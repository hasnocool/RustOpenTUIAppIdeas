# DEV-002 — RepoTUI

**Status:** seed  
**Purpose:** Portfolio dashboard for many local repositories, showing status, branches, commits, CI state, and project metadata.

## MVP

- Scan configured workspace roots
- Detect Git repositories
- Show dirty/clean state
- Show current branch and last commit
- Search/filter repositories
- Open a selected repository in the shell

## UX

```text
REPOSITORIES

> CivilizationClone       main    ● dirty
  ResearchedNewLLMHardware main   ✓ clean
  OffGridOS               main    ✓ clean
  BatteryLab              dev     ● dirty

Enter=details  o=open  r=refresh  /=filter
```

## Architecture

Repository discovery runs asynchronously and should be cancellable. Per-repository metadata can be cached and refreshed independently.

## Stretch

GitHub integration, PR/issue summaries, CI status, TODO aggregation, dependency health, and workspace graphs.
