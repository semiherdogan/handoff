# Guide

This guide explains the normal `handoff` workflow in a little more detail than the main README.

If you want a concrete example, read [Worked Example](./worked-example.md).
If you want the system model, read [Architecture](./architecture.md).
If you hit a blocked workspace, read [Troubleshooting](./troubleshooting.md).

## Default Workflow

Use this path for most features:

```bash
handoff init my-feature
# edit .handoff/current/FEATURE.md
handoff run --copy
handoff next
```

What each step does:

1. `handoff init`
   Creates or selects a feature workspace under `.handoff/features/<feature-name>/` and scans the repo for context readiness.
2. Edit `FEATURE.md`
   Describe the goal, constraints, deliverables, and acceptance criteria.
3. `handoff run --copy`
   Loads the active feature state and emits the right next prompt. It chooses planning when `STATE.md` is not ready and execution when the saved plan is ready.
4. `handoff next`
   Shows the next task or blocking action without generating a prompt.
5. `handoff continue --copy`
   Remains available when you want the continuation prompt directly instead of letting `handoff run` decide.

If `handoff init` reports missing high-value context such as `README.md` or `AGENTS.md`, run:

```bash
handoff prompt context --copy
```

Use that prompt to improve repository context only when it will materially help future AI sessions.

## Planning-Heavy Workflow

Use this path when you want review points before implementation:

```bash
handoff init my-feature
# edit .handoff/current/FEATURE.md
handoff spec --copy
handoff design --copy      # optional
handoff tasks --copy
handoff start --copy
```

## Which Command Should You Use?

Use this decision rule:

- `handoff run` is the default. Use it when you want `handoff` to inspect saved state and choose the next prompt for you.
- `handoff generate` is for planning-only refresh when you do not want implementation to start yet.
- `handoff start` is for direct execution when a valid plan already exists.
- `handoff continue` is for direct continuation when execution is already underway.
- `handoff spec`, `handoff design`, and `handoff tasks` are for manual planning checkpoints when you want explicit review steps.
- `handoff prompt <kind>` is for raw prompt selection when you already know exactly which prompt you want.

## Status and Validation

- `handoff status` makes the saved state visible: active feature, plan validity, continuity signal, current progress, artifact readiness, and the next command/focus.
- `handoff next` is the faster version when you only care about the immediate next task, block, and the prompt mode `handoff run` will choose.
- `handoff validate` performs an explicit pass/fail check for the current execution plan.

Use `handoff validate` when you want a hard answer to one question: "Is this feature ready to execute from `STATE.md`?"

## What Good Repository Context Looks Like

`README.md`, `AGENTS.md`, and `CLAUDE.md` can all help, but they should do more than exist.

They are most useful when they document:

- build, test, and run commands
- repo structure and module boundaries
- project-specific conventions
- important invariants and guardrails
- high-value troubleshooting notes

If those files are thin, stale, or generic, later AI sessions still lose time.

## Git Tip

If you want your AI assistant to see `.handoff/` locally without committing it:

```bash
handoff ignore
```

Run it again to remove the entry from `.git/info/exclude`.
