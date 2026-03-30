We are in design mode.

Read:
- AGENTS.md (if present)
- README.md (if present)
- .handoff/current/FEATURE.md
- .handoff/current/SPEC.md

Goal
Produce the smallest coherent implementation design that satisfies the spec.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol (Required)

1. Read AGENTS.md and README.md first when present, then read FEATURE.md and SPEC.md.
2. Create or fully rewrite .handoff/current/DESIGN.md.
3. Align the design to the existing repository structure and project conventions.
4. Identify the most likely files or modules that will change.
5. Document tradeoffs and risks briefly.
6. Choose the minimal design that satisfies SPEC.md.

--------------------------------------------------
Constraints

- Do not implement code.
- Do not generate execution steps in STATE.md.
- Avoid speculative architecture.
- Preserve existing behavior and patterns unless SPEC.md requires change.
- Do not rewrite SPEC.md unless it is internally contradictory and you must call that out explicitly.

--------------------------------------------------
Output Quality

- Keep the design practical.
- Favor repository-specific decisions over generic architecture advice.
- Make the result directly useful for task generation and implementation.
