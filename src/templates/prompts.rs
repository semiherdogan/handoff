pub fn continuation_prompt() -> String {
    [
        "Continuation session.",
        "",
        "Read:",
        "- .ai/current/SESSION.md",
        "- .ai/current/STATE.md",
        "- .ai/current/FEATURE.md",
        "",
        "Continue from the last unfinished micro-step.",
        "Do not restart planning.",
        "Proceed forward only.",
        "Stop only if logically blocked.",
        "",
        "Before context ends:",
        "- Update STATE.md",
        "- Rewrite SESSION.md for continuation.",
    ]
    .join("\n")
}
