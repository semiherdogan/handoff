We are in design mode.

Read:
- AGENTS.md (if present)
- README.md (if present)
- .handoff/current/FEATURE.md
- .handoff/current/SPEC.md
- .handoff/current/DECISIONS.md (if present)

Goal
Produce the smallest coherent implementation design that satisfies the spec.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol

1. Read AGENTS.md and README.md first when present, then read .handoff/current/FEATURE.md and .handoff/current/SPEC.md.
2. Create or fully rewrite .handoff/current/DESIGN.md.
3. Align the design to existing repository structure and conventions.
4. Identify the most likely files or modules that will change.
5. Document tradeoffs and risks briefly.
6. Choose the minimal design that satisfies SPEC.md.
7. Update .handoff/current/DECISIONS.md only for durable architecture or product choices.

--------------------------------------------------
Constraints

- Do not implement code or generate .handoff/current/STATE.md.
- Do not rewrite .handoff/current/SPEC.md unless it is internally contradictory and you must call that out explicitly.
- Avoid speculative architecture.
- Preserve existing behavior unless SPEC.md requires change.
- Favor repository-specific decisions over generic architecture advice.
