# Troubleshooting

This page covers the common ways a `handoff` workspace gets blocked and how to recover quickly.

## No active feature

Error:

```text
No active feature found. Run: handoff init
```

Fix:

```bash
handoff init my-feature
```

Or switch back to an existing feature:

```bash
handoff list
handoff switch my-feature
```

## `handoff run` keeps choosing planning

Cause:

- `STATE.md` is missing
- planning artifacts are incomplete
- the execution plan is invalid

Check:

```bash
handoff status
handoff validate
```

If the plan is not ready yet, use:

```bash
handoff generate --copy
```

or the manual planning path:

```bash
handoff spec --copy
handoff design --copy
handoff tasks --copy
```

## `handoff continue` says execution plan is not initialized

Error:

```text
Execution plan not initialized. Run `handoff start` first.
```

Cause:

`STATE.md` does not contain a ready execution plan yet.

Fix:

```bash
handoff start --copy
```

If that still cannot execute, finish planning first.

## Multiple current steps

Error:

```text
Invalid execution plan: multiple current steps ([>]) found.
```

Cause:

`STATE.md` has more than one active step.

Fix:

- mark finished steps as `[x]`
- keep only one `[>]`
- leave all future work as `[ ]`

## No remaining steps

Error:

```text
No remaining steps to continue.
```

Cause:

The execution plan is already complete.

Fix:

- review `SESSION.md` and finalize the feature summary
- start a new feature if more work is needed

## The assistant drifted from the repo

Symptoms:

- the prompt assumes files or conventions that do not exist
- the plan no longer matches the repository
- continuation feels stale after external edits

Fix:

- read `README.md`, `AGENTS.md`, and `CLAUDE.md` again if present
- inspect `.handoff/current/SESSION.md`
- use `handoff status` and `handoff next`
- regenerate planning if the feature intent changed materially

## `README.md`, `AGENTS.md`, or `CLAUDE.md` exist, but sessions are still weak

Cause:

Presence is not the same as quality.

Weak context files usually miss:

- build and test commands
- repo structure
- architecture constraints
- common failure modes
- project-specific conventions

Use:

```bash
handoff prompt context --copy
```

to improve repository context without implementing code.

## Templates are too generic

If the default prompts do not fit your repo, export them:

```bash
handoff export
```

This copies the embedded defaults into:

```text
.handoff/templates/
```

You can then customize them per repository.

## `.handoff/` should stay local

If you do not want to commit `.handoff/` but still want local visibility:

```bash
handoff ignore
```

Run it again to remove the exclude entry.
