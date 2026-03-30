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
3. Run `handoff run --copy` and paste the prompt into your AI assistant.
4. Let `handoff` switch between planning and execution prompts based on the saved state.
5. Run `handoff next` any time you want the next task without generating a prompt.

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
handoff run --copy
handoff next
handoff status
```

If you just want the fastest path, that is enough.

## Learn More

- [Guide](./docs/guide.md): normal workflows, planning-heavy flow, model usage pattern
- [Reference](./docs/reference.md): command list, workspace layout, config, status/validate, shell completions
- [Changelog](./CHANGELOG.md): user-facing changes

## Model Usage Pattern

`handoff` is model-agnostic, but a split-model workflow often works well:

1. Use a stronger reasoning model for planning-oriented commands such as `handoff run`, `handoff generate`, `handoff spec`, `handoff design`, and `handoff tasks` when the feature still needs planning.
2. Use a cheaper or faster coding model for execution-oriented commands such as `handoff run`, `handoff start`, and `handoff continue` once the plan is ready.
3. Switch back to the stronger planning model if implementation drifts, the plan becomes inconsistent, or you need to regenerate planning artifacts.

Example pattern:

```text
Strong planning model  -> handoff run / generate / spec / design / tasks
Cheaper coding model   -> handoff run / start / continue
```

This keeps planning quality high while reducing cost and latency during implementation loops.

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

## Contributing

Focused, minimal changes are preferred. If you change behavior or workflow contracts, also update `README.md`, `CHANGELOG.md`, and relevant templates. See [AGENTS.md](./AGENTS.md) for repository-specific contributor guidance.

## License

MIT. See [LICENSE](./LICENSE).
