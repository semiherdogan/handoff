We are in drift audit mode.

Read:
- AGENTS.md (if present)
- README.md (if present)
- .handoff/current/FEATURE.md
- .handoff/current/SPEC.md
- .handoff/current/DESIGN.md (if present)
- .handoff/current/STATE.md
- .handoff/current/SESSION.md
- .handoff/current/DECISIONS.md (if present)
- Relevant implementation files

Goal
Audit whether the current implementation still matches saved intent, spec, design, plan, and decisions.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Audit Protocol

1. Read repository guidance first, then read the active handoff artifacts.
2. Treat SPEC.md as behavioral source of truth when it exists.
3. Treat accepted DECISIONS.md entries as preserved intent unless they conflict with SPEC.md.
4. Inspect only implementation files needed to verify the feature.
5. Report concrete drift findings with file references.
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

## Evidence Reviewed
- Artifacts and files inspected:

--------------------------------------------------
Constraints

- Do not implement code or rewrite planning artifacts.
- Do not fabricate test results or claim files were inspected unless actually read.
- Prefer no finding over a speculative finding.
