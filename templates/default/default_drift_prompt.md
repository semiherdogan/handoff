We are in drift audit mode.

Read:
- AGENTS.md (if present)
- README.md (if present)
- Project documentation under docs/ when relevant
- .handoff/current/FEATURE.md
- .handoff/current/SPEC.md
- .handoff/current/DESIGN.md (if present)
- .handoff/current/STATE.md
- .handoff/current/SESSION.md
- .handoff/current/DECISIONS.md (if present)
- Relevant implementation files

Goal
Audit whether the current implementation still matches the saved feature intent, spec, design, plan, and decision history.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Audit Protocol (Required)

1. Read repository guidance first, then read the active handoff artifacts.
2. Treat SPEC.md as the behavioral source of truth when it exists.
3. Treat accepted DECISIONS.md entries as preserved intent unless they conflict with SPEC.md.
4. Inspect only implementation files needed to verify the feature.
5. Report concrete drift findings with file references when possible.
6. Separate confirmed drift from uncertainty.
7. Recommend the smallest repair path for each confirmed drift.

--------------------------------------------------
Output Format

## Drift Summary
- Overall status: No confirmed drift | Confirmed drift | Inconclusive

## Confirmed Drift
- Requirement / decision:
- Implementation evidence:
- Impact:
- Suggested repair:

## Unverified or Inconclusive
- Item:
- Why it could not be confirmed:
- What to inspect next:

## Evidence Reviewed
- Handoff artifacts:
- Repository files:
- Commands/tests, if any:

--------------------------------------------------
Constraints

- Do not implement code.
- Do not rewrite planning artifacts.
- Do not fabricate test results or claim files were inspected unless actually read.
- Prefer no finding over a speculative finding.
