# Architecture

This document explains the small set of ideas that make `handoff` work.

## Core Model

`handoff` is a local-first workflow layer for AI-assisted development.

It does not execute a cloud memory service. It does not maintain hidden agent state. It keeps the project state in plain files under `.handoff/`.

The core unit is a feature workspace:

```text
.handoff/
  config.toml
  current -> features/<feature-name>
  features/
    <feature-name>/
      FEATURE.md
      SPEC.md
      DESIGN.md
      STATE.md
      SESSION.md
```

## Artifact Responsibilities

- `FEATURE.md`: raw intent, constraints, and non-goals
- `SPEC.md`: normalized behavior and acceptance criteria
- `DESIGN.md`: optional technical shape when the feature needs it
- `STATE.md`: execution ledger and parser-sensitive progress markers
- `SESSION.md`: continuation-safe summary for the next AI session

The product depends on these files being explicit and durable. The files are the memory layer.

## Execution Model

There are two operating modes:

- planning
- execution

`handoff run` chooses between them from the saved workspace state.

The explicit commands let you drive the same flow manually:

- planning-oriented: `generate`, `spec`, `design`, `tasks`
- execution-oriented: `start`, `continue`

`status`, `next`, and `validate` expose the saved state without generating a prompt.

## State Contract

`STATE.md` is not free-form prose. It has invariants.

Execution markers:

- `[ ]` pending
- `[>]` current
- `[x]` completed

While work remains, exactly one `[>]` should exist.

Deterministic guards matter because continuation depends on them.

## Prompt and Template Layer

`handoff` ships with embedded defaults in `templates/default/`.

Users can export them into `.handoff/templates/` and override them locally.

This keeps the core workflow stable while allowing repository-specific prompt customization.

## Context Strategy

The tool assumes AI sessions work better when repository context is visible and durable.

That is why `handoff init` can call out missing high-value context like:

- `README.md`
- `AGENTS.md`

But repository context is separate from feature state:

- repo docs explain the system
- `.handoff/current/` explains the active feature

## Product Boundaries

`handoff` should stay:

- minimal
- deterministic
- local-first
- model-agnostic

That means the core flow should avoid:

- provider-specific orchestration
- cloud-hosted memory
- hidden adaptive state
- agent framework sprawl

## What Handoff Is Not

`handoff` is not:

- a hosted AI platform
- a background sync service
- a multi-agent runtime
- a provider-specific prompt marketplace

If a feature weakens the plain-text local state model, it is probably outside the product boundary.
