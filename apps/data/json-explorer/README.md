# DATA-001 — JSON Explorer

**Status:** seed  
**Purpose:** Browse large or unfamiliar JSON documents as an interactive tree rather than raw text.

## MVP

- Open JSON from file/stdin
- Expand/collapse objects and arrays
- Search keys and values
- Show path of selected node
- Copy/export selected subtree

## UX

```text
JSON EXPLORER

▼ server
  ▼ database
    host: localhost
    port: 5432
  ▼ redis
    host: localhost
    port: 6379
  ▼ features
    auth: true
    cache: false
```

## Technical experiment

Support very large documents without rendering the entire tree at once. Investigate lazy parsing/indexing and virtualized tree rendering.

## Stretch

JSONPath queries, diff mode, schema inspection, transformation previews, and streaming JSON.
