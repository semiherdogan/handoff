# handoff

AI coding tools lose intent.

`handoff` keeps it executable.

Local-first intent and execution layer for AI-assisted development.

It turns a feature brief into specs, design notes, decisions, execution steps, evidence, and continuation-safe prompts.

![Handoff](./cover.jpg)

## The Problem

Without a structured workflow layer:

- context disappears between chats
- you repeat the same setup and constraints
- intent gets blurred before implementation starts
- requirements and decisions drift apart
- execution drifts away from earlier decisions
- “what should I do next?” becomes manual work again

## What Handoff Is

`handoff` gives each feature a local workflow workspace:

- capture intent in `FEATURE.md`
- normalize behavior in `SPEC.md`
- preserve technical shape in `DESIGN.md`
- record durable choices in `DECISIONS.md`
- execute from `STATE.md`
- keep proof of work through evidence
- resume from `SESSION.md`
- audit drift with `handoff drift`

The workspace stays plain Markdown:

- `FEATURE.md`: the raw feature brief
- `SPEC.md`: normalized requirements
- `DESIGN.md`: optional technical design
- `STATE.md`: execution plan and progress
- `SESSION.md`: continuation-safe session summary
- `DECISIONS.md`: durable feature decisions and tradeoffs

Then it generates the right prompt for the current state so your assistant can plan, execute, continue, and audit the work without losing the thread.

**No API keys. No cloud. No provider lock-in.** It works with any AI coding assistant that accepts a text prompt.

## Quick Demo

```bash
handoff init my-feature
# edit .handoff/current/FEATURE.md
handoff run --copy
handoff next
handoff status
handoff drift --copy
```

What happens:

- `handoff init` creates the feature workspace
- `handoff init` also scans context readiness and tells you if high-value repo context such as `README.md` or `AGENTS.md` is missing
- `handoff run` chooses the right prompt from the saved state
- `handoff next` shows the next task or blocking action
- `handoff status` makes the current state visible
- `handoff drift` generates an audit prompt to compare saved intent against implementation

If the repo is missing high-value context, run:

```bash
handoff prompt context --copy
```

That prompt helps you improve `README.md`, `AGENTS.md`, `CLAUDE.md`, `docs/architecture.md`, `docs/conventions.md`, `docs/decisions/`, or other context files only when they would materially help future AI sessions.

## How It Works

1. Capture feature intent.
2. Turn intent into a spec.
3. Add design only when it helps.
4. Preserve durable decisions.
5. Generate an execution plan.
6. Execute with evidence.
7. Continue safely across sessions.
8. Audit drift before closing.

## More Than Memory

`handoff` is not just a place to store context.

It keeps the development loop structured:

- intent stays explicit
- specs become the behavioral source of truth
- decisions explain why choices were made
- execution steps stay deterministic
- evidence records what was actually validated
- drift audits catch mismatch before work is closed

## Why It Converts Better Than Ad-Hoc Prompting

- your assistant stops restarting from scratch every session
- feature intent becomes reviewable before code changes
- the next step stays explicit instead of living in chat history
- planning and execution stay separated
- decisions and validation evidence survive handoff
- you can switch models or assistants without throwing away progress

## Use It When You

- build with AI across multiple sessions
- work solo and need continuity without extra process overhead
- want intent, decisions, state, and evidence outside chat history
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
