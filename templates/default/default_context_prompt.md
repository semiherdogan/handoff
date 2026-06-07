We are in repository context improvement mode.

Goal
Improve the repository context files that future AI sessions depend on for onboarding and continuity.

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
Execution Protocol

1. Read the existing repository context sources first.
2. Improve only the missing or weak context files that would materially help future AI sessions.
3. Prefer README.md for onboarding, user flow, installation, and usage.
4. Prefer AGENTS.md for repository rules, constraints, and contributor guidance.
5. Use docs/ only for architecture or workflow detail that does not fit README.md or AGENTS.md. Prefer docs/architecture.md for durable system structure, docs/conventions.md for project-specific conventions, and docs/decisions/ for long-lived decisions broader than one feature.
6. Keep created or updated context files compact, concrete, and aligned with actual behavior.
7. Leave strong existing files alone.
8. Stop after updating context files. Do not implement code.

--------------------------------------------------
Constraints

- Do not create .handoff/current/ artifacts in this mode.
- Do not invent architecture not supported by the repository.
- Avoid documentation sprawl.
