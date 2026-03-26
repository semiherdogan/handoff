We are in task planning mode.

Read:
- .handoff/current/SPEC.md
- .handoff/current/DESIGN.md (if it exists)
- .handoff/current/STATE.md

Goal
Generate an execution-ready task list in .handoff/current/STATE.md.

--------------------------------------------------
Language Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol (Required)

1. Derive tasks from SPEC.md and optional DESIGN.md.
2. Write the execution plan into .handoff/current/STATE.md under "Execution Plan".
3. Break work into practical micro-steps.
4. Keep each step narrowly scoped and executable.

Step formatting rules (STRICT):
- Use markdown list form only:
  - "- [ ] Step description"
  - "- [>] Step description"
  - "- [x] Step description"
- Exactly one step must be marked as [>] when work remains.
- All not-yet-started steps other than the current one must be [ ].
- Do not use raw "[ ]" lines without list prefixes.

Additional requirements:
- Fully update STATE.md after creating the plan.
- Keep the plan grounded in SPEC.md.
- Avoid bundling unrelated concerns into one step.
- Prefer 5-10 micro-steps when reasonable.

--------------------------------------------------
Constraints

- Do not implement code.
- Do not overwrite FEATURE.md.
- Do not rewrite SPEC.md or DESIGN.md unless blocked by contradictions that must be called out explicitly.
