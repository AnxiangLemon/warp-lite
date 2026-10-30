use warp_cli::agent::Harness;

pub fn brand_color() -> pathfinder_color::ColorU {
    pathfinder_color::ColorU::new(0, 0, 0, 255)
}

pub fn display_name(harness: Harness) -> &'static str {
    match harness {
        Harness::Oz => "Warp",
        Harness::Claude => "Claude Code",
        Harness::OpenCode => "OpenCode",
        Harness::Gemini => "Gemini CLI",
        Harness::Codex => "Codex",
        Harness::Unknown => "Unknown",
    }
}

pub fn icon_for(_name: &str) -> warp_core::ui::Icon {
    warp_core::ui::Icon::HelpCircle
}
