# handoff

Lightweight, model-agnostic autonomous dev loop manager.

Local-first prompt generator — no provider API calls, no network required at runtime.

![Handoff](./cover.jpg)

## What This Is

`handoff` gives AI coding sessions a small local workspace so planning and execution can survive across multiple chats.

Instead of starting every session from scratch, you keep a feature folder with a few markdown artifacts:

- `FEATURE.md`: what you want
- `SPEC.md`: normalized requirements
- `DESIGN.md`: optional technical design
- `STATE.md`: execution plan and progress
- `SESSION.md`: continuation-safe session summary

`handoff` then generates prompts like `generate`, `start`, and `continue` so your assistant can plan, execute, and resume without drifting.

**No API keys. No cloud. No vendor lock-in.** It works with any AI coding assistant that accepts a text prompt.

## Use It When You

- Work on features that span multiple AI sessions
- Want deterministic, structured progress tracking across conversations
- Need to hand off context between different assistants or team members
- Run autonomous dev loops and need guardrails to prevent drift

## How It Works

1. Create a feature workspace with `handoff init`.
2. Describe the feature in `FEATURE.md`.
3. Run `handoff generate --copy` and paste the prompt into your AI assistant to build or refresh the planning artifacts.
4. Run `handoff start --copy` to begin implementation from the ready plan.
5. Run `handoff continue --copy` in later sessions to resume from the saved state.

That is the whole idea: a small local workspace plus prompt generation for planning, execution, and continuation.

## Installation

```bash
cargo build --release
sudo mv ./target/release/handoff /usr/local/bin/handoff
```

Or use the [latest GitHub Release](https://github.com/semiherdogan/handoff/releases).

> **macOS:** If macOS blocks the binary, allow it from **System Settings → Privacy & Security**.

## Quick Start

```bash
handoff init my-feature
# edit .handoff/current/FEATURE.md
handoff generate --copy
handoff start --copy
handoff continue --copy
handoff status
```

If you just want the fastest path, that is enough.

## Learn More

- [Guide](./docs/guide.md): normal workflows, planning-heavy flow, model usage pattern
- [Reference](./docs/reference.md): command list, workspace layout, config, status/validate, shell completions
- [Changelog](./CHANGELOG.md): user-facing changes

## Why It Helps

AI assistants are good at local execution and bad at long-lived continuity. `handoff` gives them a stable, explicit workspace so:

- planning does not need to be repeated every session
- execution can continue from a real step list
- session context is preserved in plain local files
- you can switch assistants without losing structure

## Tip

To exclude `.handoff/` from Git without affecting `.gitignore`:

> ```bash
> handoff ignore
> ```
Run it again to remove the entry from `.git/info/exclude`.
