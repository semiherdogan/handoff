# AGENTS.md

Guidelines for AI/code agents and contributors working in this repository.

## Project Snapshot

- Project: `handoff`
- Type: local-first CLI tool for autonomous dev-loop prompt generation
- Runtime model: no provider/network API calls in core flow
- Language: Rust

## Core Intent

`handoff` manages a structured feature workspace under `.handoff/` and generates prompts (`start`, `spec`, `design`, `tasks`, `continue`) for coding assistants.

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
- `docs/`: guide and reference documentation that keep `README.md` focused on onboarding
- `.github/workflows/release.yml`: tag-triggered release pipeline

## Workspace Contract

Expected structure:

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

Artifact responsibilities:

- `config.toml`: workspace-level prompt settings such as the preferred prose language for generated prompts; missing `language` must fall back to English
- `FEATURE.md`: raw feature intent and owner constraints
- `SPEC.md`: normalized behavioral requirements and acceptance criteria
- `DESIGN.md`: technical design scaffold; may stay lightweight for simple features
- `STATE.md`: execution plan and progress markers
- `SESSION.md`: continuation-safe session summary

`.handoff/current/` is reserved for handoff workflow artifacts only:

- allowed files: `FEATURE.md`, `SPEC.md`, `DESIGN.md`, `STATE.md`, `SESSION.md`
- do not place extra project documentation, analysis notes, reports, or drafts there
- if permanent project docs are needed, put them in normal repository locations such as `docs/`, the repository root, or the closest relevant module directory

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

`STATE.md` parser-sensitive structure remains English-only unless the parser contract is intentionally changed:

- section headers like `# Current Step`, `# Execution Plan`, and `# Risks`
- execution markers `[ ]`, `[>]`, and `[x]`

## Commands (Current Surface)

- `handoff init [feature] [--force]`
- `handoff switch <feature>`
- `handoff run [--copy] [--raw]`
- `handoff generate [--copy] [--raw]`
- `handoff start [--copy] [--raw]`
- `handoff spec [--copy] [--raw]`
- `handoff design [--copy] [--raw]`
- `handoff tasks [--copy] [--raw]`
- `handoff continue [--copy] [--raw]`
- `handoff prompt [generate|start|spec|design|tasks|continue] [--copy] [--raw]`
- `handoff status`
- `handoff next`
- `handoff validate`
- `handoff version`
- `handoff list`
- `handoff clean [--force]`
- `handoff archive <feature>`
- `handoff completion <shell>`
- `handoff upgrade`
- `handoff export [--force]`
- `handoff ignore`

## Command Intent (When to Use What)

- `init`: create/select feature workspace, optionally replace existing `.handoff/current` with `--force`
- `generate`: generate a planning-only prompt that updates `SPEC.md`, optional `DESIGN.md`, `STATE.md`, and `SESSION.md` without implementing code
- `run`: inspect the active workspace state and emit the next prompt automatically (`generate`, `start`, or `continue`)
- `start`: generate an execution-only prompt; require an existing valid execution plan before implementation begins
- `spec`: generate a prompt that turns `FEATURE.md` into `SPEC.md`
- `design`: generate a prompt that turns `FEATURE.md` + `SPEC.md` into `DESIGN.md`
- `tasks`: generate a prompt that turns `SPEC.md` (+ optional `DESIGN.md`) into the `STATE.md` execution plan
- `continue`: generate continuation prompt with STATE guard checks
- `prompt`: raw prompt generator (`generate`, `start`, `spec`, `design`, `tasks`, or `continue`) without continue guard semantics
- `status`: summarize active feature state, configured workflow language, execution-plan validation, blocking reason, and artifact readiness in a compact view
- `next`: show the next task or blocking action without generating a prompt
- `validate`: explicitly validate the current execution plan and fail when it is missing or structurally invalid while still printing compact diagnostics
- `version`: print the CLI build version; `handoff --version` must match `handoff version`
- `switch` / `list`: move between feature workspaces
- `clean`: remove all non-active feature directories; with `--force`, also remove active feature and clear `.handoff/current`
- `archive`: mark feature as archived; clear `.handoff/current` if archived feature was active
- `completion`: print shell completion script for supported shells
- `upgrade`: fetch latest release from GitHub, compare versions, download and replace the current binary
- `export`: copy embedded default templates to `.handoff/templates/` for user customization; prompts for confirmation if directory has files, use `--force` to skip
- `ignore`: toggle `.handoff/` in `.git/info/exclude` (add if absent, remove if present)

## Deterministic Error Contract (Do Not Drift)

The following guard errors are contract-level and must remain deterministic unless explicitly changed:

- `Execution plan not initialized. Run \`handoff start\` first.`
- `Invalid execution plan: multiple current steps ([>]) found.`
- `No remaining steps to continue.`

Also keep active-feature errors deterministic:

- `No active feature found. Run: handoff init`

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
- Release workflow writes `version.txt` from the release tag before building artifacts.
- `build.rs` sets `HANDOFF_VERSION` from `version.txt` when present, otherwise falls back to `Cargo.toml` package metadata.
- Keep `handoff version` and `handoff --version` aligned on `HANDOFF_VERSION`.
- Keep artifact names aligned with binary/package naming (`handoff`).

## Documentation Expectations

When updating workflow behavior, also update:

- `README.md` (usage flow and command guidance)
- `CHANGELOG.md` (record user-facing changes in Keep a Changelog format)
- default templates in `templates/default/` (if prompt/state contract changes)
- start/continue/spec/design/tasks prompt contracts to require full per-step `STATE.md` updates and `SESSION.md` rewrites for continuation safety where applicable

When a task is completed and it results in a user-facing change, add an entry to `CHANGELOG.md` under the `Unreleased` section before finishing.
