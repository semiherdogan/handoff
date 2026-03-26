# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- Added doc comments to all `Command`, `PromptKind`, and `CompletionShell` enum variants in `src/cli.rs` for improved `--help` output.
- Added help text to CLI arguments (`--copy`, `--raw`, `--force`, `--follow`).
- Added a README usage pattern describing how to pair a stronger planning model with a cheaper/faster implementation model when using `handoff`.
- Added `docs/guide.md` and `docs/reference.md` to move detailed workflow and command material out of the main README.

### Changed

- Simplified `README.md` into a shorter onboarding document focused on what `handoff` is, why it exists, and the fastest happy path.


## [v0.6.1] - 2026-03-26

### Added

- Added `handoff validate` to explicitly validate the current execution plan and fail fast for uninitialized or structurally invalid `STATE.md` plans.

### Changed

- Clarified that `language` controls handoff artifact prose only and does not change code syntax, identifier naming, or repository conventions.
- Updated `handoff status` to show configured workflow language, planning readiness, and execution-plan validation details.
- Updated `handoff status` to include a concrete "Why blocked" reason when a feature still needs review, generation, or `STATE.md` repair.
- Refined `handoff status` formatting, taught `status --follow` to stop cleanly on blocked plans, and expanded `handoff validate` with compact artifact diagnostics.
- Refined `handoff status` further so `Next` is visually separated, healthy artifact blocks stay hidden, and completed execution plans classify `STATE.md` as complete instead of scaffolded.
- Tightened template registry and export tests so the full default template set is asserted exactly.


## [v0.6.0] - 2026-03-26

### Added

- Added `.handoff/config.toml` language support for prompt generation with an English fallback when `language` is missing.
- Added `handoff generate` as a planning-only prompt generator that refreshes markdown artifacts without starting implementation.

### Changed

- Updated `handoff init` to generate a default `config.toml` containing `language = "English"`.
- Updated the default `start`, `spec`, `design`, `tasks`, and `continue` prompts to instruct assistants to write prose in the configured language while preserving parser-sensitive `STATE.md` structure in English.
- Updated `handoff start` to require an existing valid execution plan and direct users to `handoff generate` when planning artifacts are not ready.
- Updated the default `FEATURE.md` template to state that `.handoff/current/` is reserved for handoff artifacts and that permanent project documentation belongs in normal repository locations.
- Clarified that `language` controls handoff artifact prose only and does not change code syntax, identifier naming, or repository conventions.

## [v0.5.1] - 2026-03-02

### Changed

- Aligned `handoff --version` with `handoff version` so both now report the build-time `HANDOFF_VERSION` value.


## [v0.5.0] - 2026-03-02

### Added

- Added planning-stage commands: `handoff spec`, `handoff design`, and `handoff tasks`.
- Added support for planning artifacts in feature workspaces:
  - `SPEC.md`
  - `DESIGN.md`
- Added default templates for:
  - `SPEC.md`
  - `DESIGN.md`
  - spec prompt generation
  - design prompt generation
  - task planning prompt generation
- Added prompt support for `handoff prompt spec`, `handoff prompt design`, and `handoff prompt tasks`.
- Added tests covering planning template registration, planning-aware start mode selection, and creation of planning artifacts in new feature workspaces.

### Changed

- Updated `handoff start` to become orchestration-aware.
- `handoff start` now:
  - reuses an existing valid execution plan when present
  - uses `SPEC.md` as the behavioral planning source when available
  - creates missing planning artifacts before implementation when needed
- Updated feature initialization so new workspaces now include:
  - `FEATURE.md`
  - `SPEC.md`
  - `DESIGN.md`
  - `STATE.md`
  - `SESSION.md`
- Expanded the default `continue` prompt so it reads optional planning artifacts (`SPEC.md` and `DESIGN.md`) as supporting context while preserving existing deterministic guard behavior.
- Updated `README.md` and `AGENTS.md` to document the planning-aware workflow and new command surface.

### Notes

- `handoff` now supports both workflows:
  - simple path: `handoff start`
  - advanced path: `handoff spec` -> `handoff design` -> `handoff tasks` -> `handoff start`
- Users can still override all default file and prompt templates through `.handoff/templates/`.
- Deterministic `continue` guard behavior remains unchanged.
