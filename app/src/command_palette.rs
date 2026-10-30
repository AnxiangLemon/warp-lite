use warpui::keymap::DescriptionContext;

use crate::util::bindings::{BindingGroup, CommandBinding};

/// Ordered list of keybindings that by default are listed at the top of the command palette.
pub const PRIORITIZED_KEYBINDINGS: &[&str] = &[
    "workspace:show_theme_chooser",
    "input:toggle_natural_language_command_search",
    "input:toggle_workflows",
    "input:search_command_history",
    "pane_group:add_right",
    "workspace:open_launch_config_save_modal",
];
pub(crate) fn is_lite_binding(binding: &CommandBinding) -> bool {
    if matches!(
        binding.group,
        Some(
            BindingGroup::WarpAi
                | BindingGroup::Workflow
                | BindingGroup::Notebooks
                | BindingGroup::Folders
                | BindingGroup::EnvVarCollection
                | BindingGroup::AutoUpdate
        )
    ) {
        return false;
    }

    if binding.name.starts_with("workspace:show_settings_") {
        return matches!(
            binding.name.as_str(),
            "workspace:show_settings_appearance_page"
                | "workspace:show_settings_features_page"
                | "workspace:show_settings_keyboard_shortcuts_page"
                | "workspace:show_settings_about_page"
        );
    }

    if binding.name == "input:toggle_input_type" {
        return false;
    }

    // Fixed bindings have no stable name, so use their materialized description.
    let identifier = if binding.name.is_empty() {
        binding.description.in_context(DescriptionContext::Default)
    } else {
        binding.name.as_str()
    };
    let identifier = identifier.to_ascii_lowercase();
    !identifier
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|part| {
            matches!(
                part,
                "account"
                    | "accounts"
                    | "agent"
                    | "agents"
                    | "ai"
                    | "auth"
                    | "billing"
                    | "branch"
                    | "changelog"
                    | "cloud"
                    | "collaboration"
                    | "conversation"
                    | "conversations"
                    | "credit"
                    | "credits"
                    | "drive"
                    | "feedback"
                    | "git"
                    | "github"
                    | "handoff"
                    | "invite"
                    | "login"
                    | "logout"
                    | "mcp"
                    | "notebook"
                    | "notebooks"
                    | "onboarding"
                    | "opencode"
                    | "orchestration"
                    | "oz"
                    | "plan"
                    | "referral"
                    | "referrals"
                    | "resource"
                    | "review"
                    | "share"
                    | "shared"
                    | "signup"
                    | "subscription"
                    | "team"
                    | "teams"
                    | "telemetry"
                    | "upgrade"
                    | "workflow"
                    | "workflows"
                    | "worktree"
            )
        })
        && ![
            "natural_language",
            "natural language",
            "sign_in",
            "sign in",
            "sign_up",
            "sign up",
            "log_out",
            "log out",
            "access_token",
            "open_in_warp",
            "open in warp",
        ]
        .iter()
        .any(|phrase| identifier.contains(phrase))
}

#[cfg(test)]
#[path = "command_palette_tests.rs"]
mod tests;
