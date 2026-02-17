# handoff

Lightweight, model-agnostic autonomous dev loop manager.

`handoff` is currently a local-first prompt generator. It does not call provider APIs and does not require network access for runtime behavior.

## Features

- `.ai/` workspace management
- multi-feature workflow with active `current` symlink
- embedded default templates with `.ai/templates/` override support
- prompt generation for `start` and `continue`
- context-window handoff via `handoff continue` prompt output
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
handoff init [feature] [--force]
handoff switch <feature>
handoff continue [--copy] [--raw]
handoff start [--copy] [--raw]
handoff prompt [start|continue] [--copy] [--raw]
handoff status
handoff list
handoff archive <feature>
```

## How to Use (Full Flow)

### 1) Initialize a feature workspace

```bash
handoff init my-feature
```

This creates/uses `.ai/features/my-feature` and points `.ai/current` to it.

### 2) Define the feature before starting execution

After `init`, update:

- `.ai/current/FEATURE.md`

Write the real goal, acceptance criteria, constraints, and context there.

### 3) Generate the start prompt and begin first session

```bash
handoff start --copy
```

Paste the output into your coding assistant conversation. This should initialize
the execution plan in `.ai/current/STATE.md` and start the first micro-step.

### 4) Continue the loop

For next iterations, use:

```bash
handoff continue --copy
```

Paste into the next message/conversation to continue from unfinished steps.

### 5) Check progress anytime

```bash
handoff status
```

Shows active feature, current step, completed/remaining step counts, and risks.

### 6) Switch features when needed

```bash
handoff list
handoff switch another-feature
```

### 7) Archive finished features

```bash
handoff archive my-feature
```

If archived feature is active, `.ai/current` symlink is cleared automatically.

## When to Use Which Command

- `handoff init [feature]`
  - Use when starting a new feature workspace.
- `handoff start`
  - Use once after defining `.ai/current/FEATURE.md`.
- `handoff continue`
  - Use for ongoing work after a plan already exists.
- `handoff prompt start|continue`
  - Use when you only want prompt text output (without loop checks from `continue`).
- `handoff status`
  - Use to inspect current execution state quickly.
- `handoff switch <feature>`
  - Use to make another feature active.
- `handoff list`
  - Use to view available features and which one is active.
- `handoff archive <feature>`
  - Use to archive completed/outdated feature work.

## Installation

Build release binary:

```bash
cargo build --release
```

Install as `handoff`:

```bash
sudo mv ./target/release/handoff /usr/local/bin/handoff
```

Optional: install as a short command like `ho`:

```bash
sudo mv ./target/release/handoff /usr/local/bin/ho
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
handoff continue --copy
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
