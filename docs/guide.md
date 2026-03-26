# Guide

This guide explains the normal `handoff` workflow in a little more detail than the main README.

## Default Workflow

Use this path for most features:

```bash
handoff init my-feature
# edit .handoff/current/FEATURE.md
handoff generate --copy
handoff start --copy
```

What each step does:

1. `handoff init`
   Creates or selects a feature workspace under `.handoff/features/<feature-name>/`.
2. Edit `FEATURE.md`
   Describe the goal, constraints, deliverables, and acceptance criteria.
3. `handoff generate --copy`
   Produces a planning-only prompt for your AI assistant to update `SPEC.md`, optional `DESIGN.md`, `STATE.md`, and `SESSION.md`.
4. `handoff start --copy`
   Produces an execution prompt that starts from the existing plan.
5. `handoff continue --copy`
   Resumes later sessions from the current plan and session context.

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

## Model Usage Pattern

`handoff` is model-agnostic, but a split-model workflow often works well:

1. Use a stronger reasoning model for planning-oriented commands such as `handoff generate`, `handoff spec`, `handoff design`, and `handoff tasks`.
2. Use a cheaper or faster coding model for execution-oriented commands such as `handoff start` and `handoff continue`.
3. Switch back to the stronger planning model if implementation drifts, the plan becomes inconsistent, or you need to regenerate planning artifacts.

Example pattern:

```text
Strong planning model  -> handoff generate / spec / design / tasks
Cheaper coding model   -> handoff start / continue
```

## Git Tip

If you want your AI assistant to see `.handoff/` locally without committing it:

```bash
handoff ignore
```

Run it again to remove the entry from `.git/info/exclude`.
