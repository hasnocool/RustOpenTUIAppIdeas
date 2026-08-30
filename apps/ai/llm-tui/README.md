# AI-001 — LLM-TUI

**Status:** exploring  
**Purpose:** A terminal chat client that can switch between local and remote model backends while exposing latency, tokens, and model metadata.

## MVP

- Backend/model selector
- Prompt editor
- Streaming response view
- Conversation history
- Cancel generation
- Usage/latency footer

## UX

```text
MODEL  qwen-local:7b                         TPS  48
────────────────────────────────────────────────────
You
Explain this Rust module.

Assistant
The module separates ...
████████████████████░░ streaming
────────────────────────────────────────────────────
Enter=send  Ctrl-C=cancel  Ctrl-P=models  ↑↓=history
```

## Architecture

Provider adapters expose a common async streaming interface. Each generation is a cancellable task. Tokens arrive over a channel and update state incrementally.

## Stretch

Prompt templates, tool calls, local context search, attachments, model comparison, conversation export, and cost accounting.

## Risks

Provider APIs differ in streaming, context, errors, and usage accounting. Keep provider-specific behavior behind adapters.

## Graduation

Graduate when it is a genuinely useful multi-backend terminal AI client rather than only a chat demo.
