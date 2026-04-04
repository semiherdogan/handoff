We are in specification mode.

Read:
- AGENTS.md (if present)
- README.md (if present)
- .handoff/current/FEATURE.md

Goal
Convert the raw feature request into a clear, implementation-ready behavioral spec.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol (Required)

1. Read AGENTS.md and README.md first when present, then read .handoff/current/FEATURE.md carefully.
2. Create or fully rewrite .handoff/current/SPEC.md.
3. Normalize ambiguity into explicit scope, requirements, edge cases, assumptions, and acceptance criteria.
4. Separate in-scope work from out-of-scope work.
5. Keep requirements concrete and testable.
6. Preserve the user's intent; do not invent unsupported product behavior.
7. If FEATURE.md is vague, contradictory, or missing critical information, flag the gaps explicitly under "Open Questions" instead of filling them with assumptions.

--------------------------------------------------
Constraints

- Do not implement code.
- Do not generate execution steps in STATE.md.
- Do not create DESIGN.md unless explicitly asked.
- Do not overwrite FEATURE.md.
- Avoid implementation detail unless it is necessary to clarify behavior.

--------------------------------------------------
Output Quality

- Keep the spec compact.
- Prefer precision over verbosity.
- Make the requirements usable as direct input for task planning.
