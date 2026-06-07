We are starting an autonomous development session from a ready execution plan.

Read:
{{read_files}}

Goal
Execute the current micro-step from the existing plan and move the feature forward without drift.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Artifact Status

{{artifact_status}}

--------------------------------------------------
Mode

{{planning_mode}}

--------------------------------------------------
Execution Protocol

{{workflow_instructions}}

After each micro-step:
- Mark it as `- [x]`, set next as `- [>]`.
- Run build/tests, fix issues before proceeding.
- Add evidence to .handoff/current/STATE.md (changed files, commands, result).
- Update .handoff/current/STATE.md fully (markers, counts, risks).
- Rewrite .handoff/current/SESSION.md for safe continuation.
- Update .handoff/current/DECISIONS.md only when durable choices change.
- Continue automatically. Stop only if blocked.

.handoff/current/STATE.md markers (strict):
- `- [ ]` pending | `- [>]` current (exactly one) | `- [x]` completed
- Always use list prefix. No raw `[ ]` lines.

--------------------------------------------------
Constraints

- Do not refactor unrelated modules or restart planning unless required.
- Do not recreate .handoff/current/SPEC.md, .handoff/current/DESIGN.md, or .handoff/current/STATE.md if already coherent.
- Do not fabricate test results or skip validation.
- Do not assume unstated architecture. Ask if critical info is missing.
- Prefer evidence from planning artifacts and repo context over familiarity.
- If SPEC.md exists, treat it as behavioral source of truth over raw FEATURE.md.

--------------------------------------------------
Engineering Guardrails

- Follow existing project patterns. Preserve existing behavior unless required to change.
- Fix root causes. Do not swallow errors silently.
- Do not upgrade dependencies or weaken security unless required.
- Update tests when behavior changes. Remove only provably unused code.

--------------------------------------------------
Before Context Ends

- Ensure exactly one `[>]` exists if work remains.
- Update .handoff/current/STATE.md, rewrite .handoff/current/SESSION.md, update .handoff/current/DECISIONS.md if decisions changed.
- If AGENTS.md exists and contracts changed, update it.
- When all steps are `[x]`, write a final summary in .handoff/current/SESSION.md. Do not mark complete if tests fail or known issues remain.
