# handoff

AI coding tools forget your project.

`handoff` doesn't.

Local-first project memory layer for AI-assisted development.

It keeps a feature workspace on disk so your spec, plan, progress, and continuation context survive across sessions.

![Handoff](./cover.jpg)

## The Problem

Without a memory layer:

- context disappears between chats
- you repeat the same setup and constraints
- execution drifts away from earlier decisions
- “what should I do next?” becomes manual work again

## What Handoff Is

`handoff` gives each feature a small local workspace:

- `FEATURE.md`: the raw feature brief
- `SPEC.md`: normalized requirements
- `DESIGN.md`: optional technical design
- `STATE.md`: execution plan and progress
- `SESSION.md`: continuation-safe session summary

Then it generates the right prompt for the current state so your assistant can plan, execute, and continue without losing the thread.

**No API keys. No cloud. No provider lock-in.** It works with any AI coding assistant that accepts a text prompt.

## Quick Demo

```bash
handoff init my-feature
# edit .handoff/current/FEATURE.md
handoff run --copy
handoff next
handoff status
```

What happens:

- `handoff init` creates the feature workspace
- `handoff init` also scans context readiness and tells you if `README.md` or `AGENTS.md` are missing
- `handoff run` chooses the right prompt from the saved state
- `handoff next` shows the next task or blocking action
- `handoff status` makes the current state visible

If the repo is missing high-value context, run:

```bash
handoff prompt context --copy
```

That prompt helps you improve `README.md`, `AGENTS.md`, or other context files only when they would materially help future AI sessions.

## How It Works

1. Capture the feature intent in `FEATURE.md`.
2. Shape it into a spec and execution plan.
3. Track execution in `STATE.md` and `SESSION.md`.
4. Continue later from the saved project state.

## Why It Converts Better Than Ad-Hoc Prompting

- your assistant stops restarting from scratch every session
- the next step stays explicit instead of living in chat history
- planning and execution stay separated
- you can switch models or assistants without throwing away progress

## Use It When You

- build with AI across multiple sessions
- work solo and need continuity without extra process overhead
- want deterministic state instead of chat-based memory
- hand off work between assistants or teammates

## Installation

```bash
cargo build --release
sudo mv ./target/release/handoff /usr/local/bin/handoff
```

Or use the [latest GitHub Release](https://github.com/semiherdogan/handoff/releases).

> **macOS:** If macOS blocks the binary, allow it from **System Settings → Privacy & Security**.

## Learn More

- [Guide](./docs/guide.md): normal workflows, command choice, planning-heavy flow, model usage pattern
- [Worked Example](./docs/worked-example.md): end-to-end example from `FEATURE.md` to continued execution
- [Architecture](./docs/architecture.md): core concepts, state model, and product boundaries
- [Troubleshooting](./docs/troubleshooting.md): common failure modes and concrete recovery steps
- [Reference](./docs/reference.md): command list, workspace layout, config, status/validate, shell completions
- [Changelog](./CHANGELOG.md): user-facing changes

## Who It Is For

- solo developers shipping with AI
- indie hackers building in public
- teams that want a plain-text project memory layer around coding assistants

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

## Start Here

```bash
handoff init my-feature
```

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
