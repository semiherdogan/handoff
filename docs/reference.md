# Reference

## Commands

| Command | Description |
|---|---|
| `init [feature] [--force]` | Create or select a feature workspace |
| `run [--copy] [--raw]` | Load the active state and emit the next prompt automatically (`generate`, `start`, or `continue`) |
| `generate [--copy] [--raw]` | Generate a planning-only prompt that refreshes markdown artifacts without coding |
| `start [--copy] [--raw]` | Generate an execution prompt only when a valid execution plan already exists |
| `spec [--copy] [--raw]` | Generate a prompt to create or rewrite `SPEC.md` |
| `design [--copy] [--raw]` | Generate a prompt to create or rewrite `DESIGN.md` |
| `tasks [--copy] [--raw]` | Generate a prompt to create or rewrite the `STATE.md` execution plan |
| `continue [--copy] [--raw]` | Generate a continuation prompt (with state guards) |
| `prompt generate\|start\|spec\|design\|tasks\|continue [--copy] [--raw]` | Raw prompt output (no guard checks) |
| `status [--follow]` | Show current execution state, configured language, and execution-plan validation (`--follow` polls live) |
| `next` | Show the next task or blocking action for the active feature |
| `validate` | Validate the current execution plan and report whether it is ready, complete, or invalid |
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

## Config

`config.toml` currently supports:

```toml
language = "English"
```

If `language` is missing, `handoff` falls back to English when generating prompts.

The language setting applies to handoff prompt prose and markdown artifacts such as `FEATURE.md`, `SPEC.md`, `DESIGN.md`, and `SESSION.md`. It does not tell the assistant to rename identifiers, change code conventions, or switch programming language syntax.

Parser-sensitive `STATE.md` structure remains in English.

## Status and Validate

`handoff status` reports:

- active feature
- configured workflow language
- planning status
- execution-plan validation
- a blocking reason when applicable
- progress summary
- next recommended action

`handoff validate` gives a more explicit execution-plan check and exits with failure for uninitialized or structurally invalid plans.

## Shell Completions

```bash
# Generate and load dynamically (zsh)
source <(handoff completion zsh)

# Or persist to a completions directory
handoff completion zsh > ~/.zsh/completions/_handoff
```

If using an alias such as `ho`, map it with `compdef _handoff ho`.

## Development

```bash
cargo build          # build
cargo run -- <cmd>   # run via cargo
cargo test           # run tests
```
