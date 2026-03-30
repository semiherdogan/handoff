# Guide

This guide explains the normal `handoff` workflow in a little more detail than the main README.

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
   Creates or selects a feature workspace under `.handoff/features/<feature-name>/`.
2. Edit `FEATURE.md`
   Describe the goal, constraints, deliverables, and acceptance criteria.
3. `handoff run --copy`
   Loads the active feature state and emits the right next prompt. It chooses planning when `STATE.md` is not ready and execution when the saved plan is ready.
4. `handoff next`
   Shows the next task or blocking action without generating a prompt.
5. `handoff continue --copy`
   Remains available when you want the continuation prompt directly instead of letting `handoff run` decide.

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

## Status and Validation

- `handoff status` gives a compact overview of the active feature, workflow language, planning state, execution-plan validation, and next action.
- `handoff validate` performs an explicit pass/fail check for the current execution plan.

Use `handoff validate` when you want a hard answer to one question: "Is this feature ready to execute from `STATE.md`?"

## Git Tip

If you want your AI assistant to see `.handoff/` locally without committing it:

```bash
handoff ignore
```

Run it again to remove the entry from `.git/info/exclude`.
