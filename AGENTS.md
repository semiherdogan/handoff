# AGENTS.md

Guidelines for AI/code agents and contributors working in this repository.

## Project Snapshot

- Project: `handoff`
- Type: local-first CLI tool for autonomous dev-loop prompt generation
- Runtime model: no provider/network API calls in core flow
- Language: Rust

## Core Intent

`handoff` manages a structured feature workspace under `.ai/` and generates prompts (`start` / `continue`) for coding assistants.

The tool should remain:

- minimal
- deterministic
- local-first

## Repository Map (Where Things Live)

- `src/main.rs`: CLI entrypoint and command dispatch
- `src/cli.rs`: clap command/flag definitions
- `src/commands/`: per-command behavior (`init`, `start`, `continue`, `prompt`, `status`, etc.)
- `src/core/`: workspace, feature file handling, state parsing/guards
- `src/templates/`: template manager + prompt resolvers
- `templates/default/`: embedded default markdown templates
- `.github/workflows/release.yml`: tag-triggered release pipeline

## Workspace Contract

Expected structure:

```text
.ai/
  config.toml
  current -> features/<feature-name>
  features/
    <feature-name>/
      FEATURE.md
      STATE.md
      SESSION.md
```

## STATE.md Invariants (Important)

Execution plan markers:

- `[ ]` pending
- `[>]` current
- `[x]` completed

Accepted plan line forms:

- `- [ ] step`
- `* [>] step`
- `1. [x] step`

Guard behavior for `continue`:

- fail if execution plan is not initialized
- fail if multiple `[>]` exist
- fail if there are no remaining steps

Deterministic guard errors are part of the contract; do not silently relax them.

## Commands (Current Surface)

- `handoff init [feature] [--force]`
- `handoff switch <feature>`
- `handoff start [--copy] [--raw]`
- `handoff continue [--copy] [--raw]`
- `handoff prompt [start|continue] [--copy] [--raw]`
- `handoff status`
- `handoff version`
- `handoff list`
- `handoff clean`
- `handoff archive <feature>`
- `handoff completion <shell>`

## Command Intent (When to Use What)

- `init`: create/select feature workspace, optionally replace existing `.ai/current` with `--force`
- `start`: generate initial prompt for first session
- `continue`: generate continuation prompt with STATE guard checks
- `prompt`: raw prompt generator (`start` or `continue`) without continue guard semantics
- `status`: summarize active feature state
- `version`: print CLI version from Cargo package metadata
- `switch` / `list`: move between feature workspaces
- `clean`: remove all non-active feature directories
- `archive`: mark feature as archived; clear `.ai/current` if archived feature was active
- `completion`: print shell completion script for supported shells

## Deterministic Error Contract (Do Not Drift)

The following guard errors are contract-level and must remain deterministic unless explicitly changed:

- `Execution plan not initialized. Run \`ai start\` first.`
- `Invalid execution plan: multiple current steps ([>]) found.`
- `No remaining steps to continue.`

Also keep active-feature errors deterministic:

- `No active feature found. Run: ai init`

## Change Rules for Agents

1. Keep changes focused and minimal.
2. Preserve existing behavior unless explicitly asked to change it.
3. Do not add provider abstractions, plugin systems, or unrelated architecture.
4. Follow existing error handling style (`anyhow` + contextual messages).
5. Add/update tests when changing behavior or parser/guard logic.
6. Avoid refactoring unrelated modules.
7. Keep release workflow and README in sync with command/binary naming.

## First 5 Minutes Checklist for a New Agent

1. Read `README.md` for user flow.
2. Confirm CLI surface in `src/cli.rs` and dispatch in `src/main.rs`.
3. Read `src/core/state.rs` invariants before changing continue/status behavior.
4. If modifying prompts/contracts, check `templates/default/` and keep docs in sync.
5. Run `cargo check` before handing off.

## Validation Checklist Before Finishing

- Run `cargo check` for all code changes.
- Run `cargo test` when changing parsing/guards/command behavior.
- Ensure README and templates are updated when workflow contracts change.
- Keep release artifact naming aligned with binary/package naming (`handoff`).

## Release Notes

- GitHub Actions release workflow is in `.github/workflows/release.yml`.
- Releases are triggered by pushing tags like `v0.1.0`.
- Release workflow updates `Cargo.toml` package version from the release tag before building artifacts.
- Keep artifact names aligned with binary/package naming (`handoff`).

## Documentation Expectations

When updating workflow behavior, also update:

- `README.md` (usage flow and command guidance)
- default templates in `templates/default/` (if prompt/state contract changes)
- start/continue prompt contract to require full per-step `STATE.md` updates and `SESSION.md` rewrites for continuation safety
