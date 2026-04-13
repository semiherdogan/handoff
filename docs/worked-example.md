# Worked Example

This example shows the full `handoff` loop on a small feature so the workflow is concrete instead of abstract.

## Scenario

You want to add a `--json` flag to an existing CLI command.

## 1. Initialize the feature

```bash
handoff init json-output
```

This creates the workspace:

```text
.handoff/
  current -> features/json-output
  features/json-output/
    FEATURE.md
    SPEC.md
    DESIGN.md
    STATE.md
    SESSION.md
```

## 2. Write the feature brief

Start with `.handoff/current/FEATURE.md`.

Example:

```md
# Feature: json-output

## Goal
Add a `--json` flag to `handoff status` so scripts can consume the current state.

## Requirements
- Preserve the existing human-readable output by default.
- Return deterministic JSON keys.
- Fail with a clear error if no active feature exists.

## Non-Goals
- Do not redesign `handoff status`.
- Do not add network behavior.
```

## 3. Use the default workflow

For most features, use:

```bash
handoff run --copy
```

`handoff run` decides what prompt comes next:

- if planning is incomplete, it emits a planning prompt
- if the execution plan is ready, it emits an execution prompt

Use:

```bash
handoff next
handoff status
```

to see the current state without generating another prompt.

## 4. Planning artifacts take shape

After planning, the feature workspace should converge toward:

- `SPEC.md`: clear behavioral requirements and acceptance criteria
- `DESIGN.md`: optional implementation notes when the change is complex enough
- `STATE.md`: execution plan with exactly one `[>]` current step while work remains
- `SESSION.md`: continuation-safe summary of what changed and what comes next

Example execution plan:

```md
# Current Step
Implement JSON serialization for `handoff status`.

# Execution Plan
- [x] Add CLI flag parsing for `--json`.
- [>] Return JSON output from the status command.
- [ ] Add tests for no-active-feature and healthy-status cases.
```

## 5. Execute with continuity

Once the plan is ready, either:

```bash
handoff run --copy
```

or:

```bash
handoff start --copy
handoff continue --copy
```

Use `run` when you want `handoff` to choose automatically.
Use `start` and `continue` when you want to drive the execution flow directly.

## 6. Keep artifacts current during execution

During implementation:

- mark completed steps as `[x]`
- keep exactly one `[>]` if work remains
- rewrite `SESSION.md` after meaningful progress
- use `handoff validate` when you need a hard readiness check

## 7. Finish cleanly

When all planned steps are done:

- every execution step should be `[x]`
- `SESSION.md` should summarize what shipped and any follow-up items
- `handoff next` should stop pointing to more execution work

## Why This Example Matters

The product is not just “prompt generation.”

The value is the saved state on disk:

- the feature brief survives
- the plan survives
- the execution ledger survives
- the continuation context survives

That is what lets a later AI session resume without starting over.
