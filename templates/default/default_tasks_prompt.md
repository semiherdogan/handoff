We are in task planning mode.

Read:
- AGENTS.md (if present)
- README.md (if present)
- .handoff/current/SPEC.md
- .handoff/current/DESIGN.md (if it exists)
- .handoff/current/STATE.md
- .handoff/current/DECISIONS.md (if present)

Goal
Generate an execution-ready task list in .handoff/current/STATE.md.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol

1. Read AGENTS.md and README.md first when present, then derive tasks from .handoff/current/SPEC.md and optional .handoff/current/DESIGN.md.
2. Write the execution plan into .handoff/current/STATE.md under "Execution Plan".
3. Break work into practical micro-steps (prefer 5-10).
4. Keep each step narrowly scoped and executable. Avoid bundling unrelated concerns into one step.
5. Exactly one step must be `[>]`. All others must be `[ ]`.
6. Keep the plan grounded in SPEC.md and aligned with DECISIONS.md.
7. If SPEC.md or DESIGN.md contain contradictions, flag them instead of guessing.

.handoff/current/STATE.md markers (strict):
- `- [ ]` pending | `- [>]` current (exactly one) | `- [x]` completed
- Always use list prefix. No raw `[ ]` lines.

--------------------------------------------------
Constraints

- Do not implement code.
- Do not overwrite .handoff/current/FEATURE.md.
- Do not rewrite .handoff/current/SPEC.md or .handoff/current/DESIGN.md unless blocked by contradictions that must be called out explicitly.
