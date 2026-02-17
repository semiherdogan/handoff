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
handoff status [--follow]
handoff version
handoff list
handoff archive <feature>
handoff completion <shell>
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
  - Add `--follow` to watch the active `[>]` step live (polls every 2s) until work completes.
- `handoff version`
  - Use to print the current CLI version from `Cargo.toml`.
- `handoff switch <feature>`
  - Use to make another feature active.
- `handoff list`
  - Use to view available features and which one is active.
- `handoff archive <feature>`
  - Use to archive completed/outdated feature work.
- `handoff completion <shell>`
  - Use to generate shell completion scripts for `bash`, `zsh`, `fish`, `powershell`, or `elvish`.

## Shell Autocomplete

Generate completion script output:

```bash
handoff completion zsh
```

Load completions dynamically in zsh (recommended for development so new commands are picked up automatically):

```bash
autoload -U compinit && compinit
source <(handoff completion zsh)
# optional alias mapping
compdef _handoff ho
```

Persist in zsh by writing to your completions directory and loading it (recommended for stable installs):

```bash
mkdir -p ~/.zsh/completions
handoff completion zsh > ~/.zsh/completions/_handoff
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc
```

If you installed the binary under another command name (for example `ho`), map it with `compdef`:

```bash
compdef _handoff ho
```

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

The release workflow updates `Cargo.toml` package version from the pushed tag (`vX.Y.Z` -> `version = "X.Y.Z"`) before building release binaries.

Release artifacts are built for:

- Linux (`x86_64-unknown-linux-gnu`)
- macOS (`aarch64-apple-darwin`)

Artifact names include the release tag suffix (for example: `handoff-aarch64-apple-darwin-v0.1.0`).

### macOS note for release binaries

If you download `handoff` from GitHub Releases, macOS may block first launch.
Allow it from **System Settings → Privacy & Security** (look for the blocked app message and click **Allow Anyway**), then run it again.
