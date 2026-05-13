# skills - signal-persona-router

*Per-repo agent guide.*

## Checkpoint - read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- `persona-router/ARCHITECTURE.md`
- `persona-introspect/ARCHITECTURE.md` when touching observation records

## What this repo owns

- Router-owned Signal vocabulary.
- Router observation records used by `persona-introspect`.
- Future router operational relations when they are extracted from consumers.

## What this repo does not own

- Router daemon code, actor logic, state reducers, or redb tables.
- Message ingress records owned by `signal-persona-message`.
- Introspection query envelopes owned by `signal-persona-introspect`.
