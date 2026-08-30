# AI-002 — ModelManager

**Status:** exploring  
**Purpose:** Inventory and manage local LLM models, their sizes, metadata, availability, and lifecycle.

## MVP

- Discover configured local model stores
- List model name, size, format, quantization, context metadata where available
- Search/filter
- Show details
- Start/stop or invoke supported backends through adapters

## UX

```text
LOCAL MODELS

> qwen-coder-7b       4.4 GB   Q4_K_M
  llama-8b             4.7 GB   Q4_K_M
  deepseek-8b          4.9 GB   Q4_K_M

Enter=details  r=run  /=filter  d=delete
```

Destructive operations require confirmation and should be backend-specific.

## Architecture

Use provider/model-store adapters and background metadata discovery. Do not block the UI while scanning model directories or querying services.

## Stretch

Hardware fit estimates, benchmark history, duplicate detection, disk usage, download queues, and model recommendations.
