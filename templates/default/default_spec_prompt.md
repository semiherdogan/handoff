We are in specification mode.

Read:
- AGENTS.md (if present)
- README.md (if present)
- .handoff/current/FEATURE.md
- .handoff/current/DECISIONS.md (if present)

Goal
Convert the raw feature request into a clear, implementation-ready behavioral spec.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol

1. Read AGENTS.md and README.md first when present, then read .handoff/current/FEATURE.md carefully.
2. Create or fully rewrite .handoff/current/SPEC.md.
3. Normalize ambiguity into explicit scope, requirements, edge cases, assumptions, and acceptance criteria.
4. Separate in-scope from out-of-scope work.
5. Keep requirements concrete and testable.
6. Preserve the user's intent; do not invent unsupported behavior.
7. Preserve accepted decisions from .handoff/current/DECISIONS.md when they affect behavior.
8. If FEATURE.md is vague or contradictory, flag gaps under "Open Questions" instead of assuming.

--------------------------------------------------
Constraints

- Do not implement code.
- Do not generate .handoff/current/STATE.md or .handoff/current/DESIGN.md.
- Do not overwrite .handoff/current/FEATURE.md.
- Keep the spec compact: precision over verbosity.
