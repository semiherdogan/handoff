We are in planning generation mode.

Read:
- .handoff/current/FEATURE.md
- .handoff/current/SPEC.md
- .handoff/current/DESIGN.md
- .handoff/current/STATE.md
- .handoff/current/SESSION.md

Goal
Create or refresh the planning markdown artifacts needed for execution, then stop before any code changes.

--------------------------------------------------
Language Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol (Required)

1. Read `.handoff/current/FEATURE.md` carefully.
2. Create or fully rewrite `.handoff/current/SPEC.md` from the feature intent.
3. Create or fully rewrite `.handoff/current/DESIGN.md` only if the feature is complex enough to benefit from explicit technical planning. Otherwise keep the file lightweight and clearly note that a detailed design is not needed.
4. Create or fully rewrite `.handoff/current/STATE.md` with an execution-ready micro-step plan grounded in SPEC.md and optional DESIGN.md.
5. Rewrite `.handoff/current/SESSION.md` so the next execution session can safely continue from the generated plan.
6. Ensure exactly one step is marked as `[>]` in `.handoff/current/STATE.md` when work remains.
7. Stop after updating the markdown artifacts. Do not implement code, edit source files, or run the execution loop.

STATE.md step formatting rules (STRICT):
- Use markdown list form only:
  - "- [ ] Step description"
  - "- [>] Step description"
  - "- [x] Step description"
- Do not use raw "[ ]" lines without list prefixes.

--------------------------------------------------
Constraints

- Do not implement code.
- Do not modify files outside `.handoff/current/*.md`.
- Preserve the user's intent from `FEATURE.md`.
- Keep the generated plan practical, concrete, and ready for execution.
