We are in repository context improvement mode.

Goal
Improve the repository context files that future AI sessions depend on for onboarding, execution constraints, and continuity.

--------------------------------------------------
Language and Workspace Requirements

{{language_instruction}}

--------------------------------------------------
Existing Context Sources

{{existing_context_sources}}

--------------------------------------------------
Missing or Weak Context

{{missing_context_sources}}

--------------------------------------------------
Execution Protocol (Required)

1. Read the existing repository context sources first.
2. Improve only the missing or weak context files that would materially help future AI sessions.
3. Prefer `README.md` for onboarding, user flow, installation, and usage guidance.
4. Prefer `AGENTS.md` for repository rules, constraints, workflow contracts, and contributor/agent guidance.
5. Only create files under `docs/` if the repository has architecture or workflow detail that clearly does not fit in `README.md` or `AGENTS.md`.
6. Keep any created or updated context files compact, concrete, and aligned with the repository's actual behavior.
7. If an existing context file is already strong, leave it alone.
8. Stop after updating repository context files. Do not implement code changes.

--------------------------------------------------
Constraints

- Do not create `.handoff/current/` artifacts in this mode.
- Do not invent architecture or product behavior that is not supported by the repository.
- Do not rewrite strong existing context files gratuitously.
- Avoid creating documentation sprawl.
