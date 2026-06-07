We are continuing an autonomous development session.

Read:
- AGENTS.md (if present)
- README.md (if present)
- .handoff/current/SESSION.md
- .handoff/current/STATE.md
- .handoff/current/FEATURE.md
- .handoff/current/SPEC.md (if present)
- .handoff/current/DESIGN.md (if present)
- .handoff/current/DECISIONS.md (if present)

Goal
Complete the remaining micro-steps defined in .handoff/current/STATE.md.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Execution Protocol

1. Continue from the last unfinished micro-step. Do not regenerate the plan unless logically blocked.
2. Implement one micro-step at a time.
3. After each step:
   - Mark completed step as `- [x]`, set next as `- [>]`.
   - Run build/tests, fix issues before proceeding.
   - Add evidence to .handoff/current/STATE.md (changed files, commands, result).
   - Update .handoff/current/STATE.md fully (markers, counts, risks).
   - Rewrite .handoff/current/SESSION.md for safe continuation.
   - Update .handoff/current/DECISIONS.md only when durable choices change.
4. Continue automatically while steps remain. Stop if blocked.

.handoff/current/STATE.md markers (strict):
- `- [ ]` pending | `- [>]` current (exactly one) | `- [x]` completed
- Always use list prefix. No raw `[ ]` lines.

--------------------------------------------------
Constraints

- Do not modify unrelated modules or introduce speculative improvements.
- Do not fabricate test results or skip validation.
- Do not assume unstated architecture. Ask if critical info is missing.
- Prefer evidence from planning artifacts and repo context over familiarity.
- If multiple interpretations exist, choose the one most consistent with FEATURE.md and state assumptions explicitly.

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
