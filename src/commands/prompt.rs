use crate::cli::PromptKind;
use crate::core::paths::AiPaths;
use crate::commands::prompt_output;
use crate::templates::manager::TemplateManager;
use crate::templates::prompts;
use anyhow::Result;

pub fn run(paths: &AiPaths, kind: Option<PromptKind>, copy: bool, raw: bool) -> Result<()> {
    let template_manager = TemplateManager::new(paths);
    let selected = kind.unwrap_or(PromptKind::Continue);

    let prompt = match selected {
        PromptKind::Start => prompts::start_prompt(
            &template_manager,
            &prompts::StartPromptContext {
                read_files: "- .handoff/current/FEATURE.md\n- .handoff/current/STATE.md\n- .handoff/current/SESSION.md\n- .handoff/current/SPEC.md (if present)\n- .handoff/current/DESIGN.md (if present)".to_owned(),
                artifact_status: "- Planning state unknown in raw prompt mode. Inspect local artifacts before acting.".to_owned(),
                planning_mode: "Raw orchestration mode. Reuse existing planning artifacts when present; otherwise create the missing planning artifacts before implementation.".to_owned(),
                workflow_instructions: "1. Inspect FEATURE.md, STATE.md, SESSION.md, and any available SPEC.md / DESIGN.md files.\n2. If STATE.md already has a valid execution plan, start from it.\n3. Otherwise create or refine SPEC.md, optionally create DESIGN.md, then generate STATE.md.\n4. Keep execution steps small, keep exactly one [>] if work remains, and update SESSION.md after each step transition.".to_owned(),
            },
        ),
        PromptKind::Spec => prompts::spec_prompt(&template_manager),
        PromptKind::Design => prompts::design_prompt(&template_manager),
        PromptKind::Tasks => prompts::tasks_prompt(&template_manager),
        PromptKind::Continue => prompts::continuation_prompt(&template_manager),
    };

    prompt_output::output_prompt(&prompt, copy, raw)
}
