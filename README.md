# handoff

Lightweight, model-agnostic autonomous dev loop manager.

Local-first prompt generator — no provider API calls, no network required at runtime.

![Handoff](./cover.jpg)

## The Problem

AI coding assistants lose context between sessions. Every time you start a new conversation, the assistant has no memory of what was done, what's in progress, or what comes next. You end up re-explaining the feature, repeating constraints, and hoping it picks up where it left off.

## What handoff Does

`handoff` keeps structured state between coding sessions so your AI assistant can continue autonomously without losing track.

It manages a local `.handoff/` workspace per feature — tracking the goal (`FEATURE.md`), optional planning artifacts (`SPEC.md`, `DESIGN.md`), execution plan and progress (`STATE.md`), and session context (`SESSION.md`). When a session ends, you run `handoff continue` and paste the generated prompt into your next conversation. The assistant picks up exactly where it left off.

**No API keys. No cloud. No vendor lock-in.** It works with any AI coding assistant — Cursor, Windsurf, Copilot, Claude, ChatGPT, or anything that accepts a text prompt.

### Use it when you

- Work on features that span multiple AI sessions
- Want deterministic, structured progress tracking across conversations
- Need to hand off context between different assistants or team members
- Run autonomous dev loops and need guardrails to prevent drift

## Features

- `.handoff/` workspace with multi-feature support and active `current` symlink
- embedded default templates with `.handoff/templates/` override support
- orchestration-aware `start` prompt generation plus explicit `spec`, `design`, and `tasks` planning prompts
- `continue` prompt generation with clipboard copy (`--copy`)
- deterministic state guardrails for safe continuation across sessions

<!-- ![Demo](./demo.gif) -->

## Installation

```bash
cargo build --release
sudo mv ./target/release/handoff /usr/local/bin/handoff
```

Or use the [latest GitHub Release](https://github.com/semiherdogan/handoff/releases).

> **macOS:** If macOS blocks the binary, allow it from **System Settings → Privacy & Security**.

## Quick Start

```bash
# 1. Create a feature workspace
handoff init my-feature

# 2. Optional: set the workspace language in .handoff/config.toml
#    Default is English. Example:
#    language = "Turkish"

# 3. Define the feature
#    Edit .handoff/current/FEATURE.md with goal, requirements, and constraints.

# 4. Generate the planning prompt
handoff generate --copy
#    Paste into your coding assistant to create or refresh
#    SPEC.md / optional DESIGN.md / STATE.md / SESSION.md only.

# 5. Generate the execution prompt
handoff start --copy
#    Paste into your coding assistant to start implementation from the ready plan.

# 6. Optional: inspect planning phases directly
handoff spec --copy
handoff design --copy
handoff tasks --copy

# 7. Continue in subsequent sessions
handoff continue --copy
#    Paste into a new conversation to resume from where you left off.

# 8. Check progress
handoff status

# 9. Archive when done
handoff archive my-feature
```

## Commands

| Command | Description |
|---|---|
| `init [feature] [--force]` | Create or select a feature workspace |
| `generate [--copy] [--raw]` | Generate a planning-only prompt that refreshes markdown artifacts without coding |
| `start [--copy] [--raw]` | Generate an execution prompt only when a valid execution plan already exists |
| `spec [--copy] [--raw]` | Generate a prompt to create or rewrite `SPEC.md` |
| `design [--copy] [--raw]` | Generate a prompt to create or rewrite `DESIGN.md` |
| `tasks [--copy] [--raw]` | Generate a prompt to create or rewrite the `STATE.md` execution plan |
| `continue [--copy] [--raw]` | Generate a continuation prompt (with state guards) |
| `prompt generate\|start\|spec\|design\|tasks\|continue [--copy] [--raw]` | Raw prompt output (no guard checks) |
| `status [--follow]` | Show current execution state (`--follow` polls live) |
| `switch <feature>` | Switch active feature |
| `list` | List available features |
| `clean [--force]` | Remove non-active features (`--force` removes all) |
| `archive <feature>` | Archive a feature (clears `current` if active) |
| `export [--force]` | Export default templates to `.handoff/templates/` for customization |
| `ignore` | Toggle `.handoff/` in `.git/info/exclude` (add if absent, remove if present) |
| `completion <shell>` | Generate shell completions (`bash`, `zsh`, `fish`, `powershell`, `elvish`) |
| `upgrade` | Upgrade to the latest GitHub release |
| `version` | Print CLI version |

## Workspace Layout

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

`config.toml` currently supports:

```toml
language = "English"
```

If `language` is missing, `handoff` falls back to English when generating prompts. The language setting applies to handoff prompt prose and markdown artifacts such as `FEATURE.md`, `SPEC.md`, `DESIGN.md`, and `SESSION.md`. It does not tell the assistant to rename identifiers, change code conventions, or switch programming language syntax. Parser-sensitive `STATE.md` structure remains in English.

## Planning Workflow

`handoff` now supports two ways to start work:

1. Default path:
   Run `handoff generate --copy` to refresh planning artifacts, then run `handoff start --copy` to begin implementation.
2. Advanced path:
   Run `handoff spec`, `handoff design`, and `handoff tasks` explicitly if you want review checkpoints before coding.

Recommended flow for most users:

```bash
handoff init my-feature
# edit .handoff/current/FEATURE.md
handoff generate --copy
handoff start --copy
```

Recommended flow for planning-heavy features:

```bash
handoff init my-feature
# edit .handoff/current/FEATURE.md
handoff spec --copy
handoff design --copy      # optional
handoff tasks --copy
handoff start --copy
```

> **Tip:** To exclude `.handoff/` from Git without affecting `.gitignore` (so your AI assistant can still see it), use:
> ```bash
> handoff ignore
> ```
> Run it again to remove the entry. This toggles `.handoff/` in `.git/info/exclude`.

## Shell Completions

```bash
# Generate and load dynamically (zsh)
source <(handoff completion zsh)

# Or persist to a completions directory
handoff completion zsh > ~/.zsh/completions/_handoff
```

If using an alias (e.g. `ho`), map it with `compdef _handoff ho`.

## Development

```bash
cargo build          # build
cargo run -- <cmd>   # run via cargo
cargo test           # run tests
```
