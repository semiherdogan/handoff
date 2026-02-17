# ai

Lightweight, model-agnostic autonomous dev loop manager.

`ai` is currently a local-first prompt generator. It does not call provider APIs and does not require network access for runtime behavior.

## Features

- `.ai/` workspace management
- multi-feature workflow with active `current` symlink
- embedded default templates with `.ai/templates/` override support
- prompt generation for `start` and `continue`
- context-window handoff via `ai continue` prompt output
- clipboard copy support (`--copy`)
- deterministic state guardrails for continuation

## Workspace Layout

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

## CLI Commands

```bash
ai init [feature] [--force]
ai switch <feature>
ai continue [--copy] [--raw]
ai start [--copy] [--raw]
ai prompt [start|continue] [--copy] [--raw]
ai status
ai list
ai archive <feature>
```

## Development

Build:

```bash
cargo build
```

Run with Cargo:

```bash
cargo run -- <command>
```

Run tests:

```bash
cargo test
```

## Context Handoff (New Conversation)

When context length is full, you can continue safely in a new conversation:

```bash
ai continue --copy
```

Then paste the copied prompt into the new conversation. The prompt points to:

- `.ai/current/SESSION.md`
- `.ai/current/STATE.md`
- `.ai/current/FEATURE.md`

This lets the next session continue from where the previous one stopped.

## Releases

This repository includes a GitHub Actions workflow that automatically creates a GitHub Release when a version tag is pushed (for example: `v0.1.0`).

Release artifacts are built for:

- Linux (`x86_64-unknown-linux-gnu`)
- macOS (`aarch64-apple-darwin`)
- Windows (`x86_64-pc-windows-msvc`)

To publish a release:

```bash
git tag v0.1.0
git push origin v0.1.0
```
