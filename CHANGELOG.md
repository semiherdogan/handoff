# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.5.0] - 2026-03-02

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
