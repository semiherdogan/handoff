We are starting a new feature in autonomous loop mode.

Read:
- .ai/current/FEATURE.md

--------------------------------------------------
Role
You are a senior engineer executing a structured implementation loop.

Goal
Analyze the feature definition and begin execution.

--------------------------------------------------
Execution Protocol (Required)

1. Analyze .ai/current/FEATURE.md.
2. Generate a structured implementation plan.
3. Break into 5–10 micro-steps (each < 30 minutes).
4. Write the plan into .ai/current/STATE.md under "Execution Plan".

   Step formatting rules (STRICT):
   - Use markdown list form only:
     - "- [ ] Step description"
     - "- [>] Step description" (exactly one current step)
     - "- [x] Step description" (completed)
   - Exactly one step must be marked as [>].
   - All others must be [ ].
   - Do NOT use raw "[ ]" lines without list prefix.

5. Set the first micro-step as "- [>] ...".
6. Begin implementing the current micro-step.

After each micro-step:
- Mark it as "- [x]".
- Mark the next step as "- [>]".
- Run build/tests.
- Fix issues before proceeding.
- Update .ai/current/STATE.md.
- Update .ai/current/SESSION.md.
- Continue automatically.

Stop only if logically blocked.

--------------------------------------------------
Constraints

- Do not refactor unrelated modules.
- Do not restart planning unless required.
- Do not introduce architectural changes unless necessary.
- Avoid speculative improvements.

--------------------------------------------------
Before Context Ends

- Ensure exactly one [>] exists if work remains.
- Update .ai/current/STATE.md.
- Rewrite .ai/current/SESSION.md for safe continuation.
