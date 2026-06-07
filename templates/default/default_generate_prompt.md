We are in planning generation mode.

Read:
- AGENTS.md (if present)
- README.md (if present)
- .handoff/current/FEATURE.md
- .handoff/current/SPEC.md
- .handoff/current/DESIGN.md
- .handoff/current/STATE.md
- .handoff/current/SESSION.md
- .handoff/current/DECISIONS.md (if present)

Goal
Create or refresh the planning artifacts needed for execution, then stop before any code changes.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol

1. Read AGENTS.md and README.md first when present, then read .handoff/current/FEATURE.md carefully.
2. Create or fully rewrite .handoff/current/SPEC.md from the feature intent.
3. Create or rewrite .handoff/current/DESIGN.md only if the feature benefits from explicit technical planning. Otherwise keep it lightweight.
4. Create or rewrite .handoff/current/STATE.md with an execution-ready micro-step plan grounded in SPEC.md and optional DESIGN.md.
5. Update .handoff/current/DECISIONS.md only for durable product or architecture decisions.
6. Rewrite .handoff/current/SESSION.md so the next session can safely continue from the generated plan.
7. Ensure exactly one step is marked `[>]` in .handoff/current/STATE.md when work remains.
8. If FEATURE.md is vague or contradictory, flag gaps in SPEC.md under "Open Questions" instead of assuming.
9. Stop after updating artifacts. Do not implement code.

.handoff/current/STATE.md markers (strict):
- `- [ ]` pending | `- [>]` current (exactly one) | `- [x]` completed
- Always use list prefix. No raw `[ ]` lines.

--------------------------------------------------
Constraints

- Do not implement code or modify files outside `.handoff/current/*.md`.
- Preserve the user's intent from .handoff/current/FEATURE.md.
- Keep the plan practical, concrete, and ready for execution.
- Do not record routine implementation choices in .handoff/current/DECISIONS.md.
