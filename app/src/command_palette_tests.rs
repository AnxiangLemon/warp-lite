use super::*;

#[test]
fn lite_palette_keeps_terminal_management_actions() {
    for name in [
        "workspace:new_tab",
        "workspace:rename_active_tab",
        "workspace:new_tab_group",
        "workspace:pin_active_tab",
        "workspace:show_settings_appearance_page",
        "workspace:show_settings_features_page",
        "workspace:show_settings_keyboard_shortcuts_page",
        "input:search_command_history",
        "pane_group:add_right",
        "terminal:copy_outputs",
        "terminal:bookmark_selected_block",
    ] {
        let binding = CommandBinding::new(name.into(), "Local action".into(), None);
        assert!(is_lite_binding(&binding), "{name}");
    }
}

#[test]
fn lite_palette_removes_services_and_hidden_settings() {
    for name in [
        "workspace:toggle_ai_assistant",
        "workspace:show_settings_account_page",
        "workspace:show_settings_billing_and_usage_page",
        "workspace:show_settings_code_page",
        "workspace:show_settings_privacy_page",
        "workspace:copy_access_token_to_clipboard",
        "workspace:trigger_auto_handoff_to_cloud",
        "workspace:log_out",
        "workspace:show_invite_modal",
        "workspace:left_panel_warp_drive",
        "input:toggle_input_type",
        "input:toggle_natural_language_command_search",
        "terminal:open_share_block_modal",
        "terminal:open_in_warp",
        "workspace:toggle_git_panel",
        "terminal:git_branch_menu",
    ] {
        let binding = CommandBinding::new(name.into(), "Service action".into(), None);
        assert!(!is_lite_binding(&binding), "{name}");
    }
}

#[test]
fn lite_palette_filters_unnamed_settings_and_feature_groups() {
    let setting = CommandBinding::new(
        String::new(),
        "Enable match AI font to terminal font".into(),
        None,
    );
    assert!(!is_lite_binding(&setting));

    let terminal_setting = CommandBinding::new(String::new(), "Enable cursor blink".into(), None);
    assert!(is_lite_binding(&terminal_setting));

    let mut workflow = CommandBinding::new("objects:open".into(), "Open".into(), None);
    workflow.group = Some(BindingGroup::Workflow);
    assert!(!is_lite_binding(&workflow));
}
