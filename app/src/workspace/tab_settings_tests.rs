use settings::Setting;
use warpui::{App, SingletonEntity};

use super::*;
use crate::test_util::settings::initialize_settings_for_tests;
use crate::workspace::header_toolbar_item::HeaderToolbarItemKind;

#[test]
fn use_latest_user_prompt_as_conversation_title_in_tab_names_defaults_to_false() {
    App::test((), |mut app| async move {
        initialize_settings_for_tests(&mut app);

        TabSettings::handle(&app).read(&app, |settings, _ctx| {
            assert!(!*settings.use_latest_user_prompt_as_conversation_title_in_tab_names);
        });
    });
}

#[test]
fn use_latest_user_prompt_as_conversation_title_in_tab_names_uses_vertical_tabs_path() {
    assert_eq!(
        UseLatestUserPromptAsConversationTitleInTabNames::toml_path(),
        Some("appearance.vertical_tabs.use_latest_prompt_as_title")
    );
    assert_eq!(
        UseLatestUserPromptAsConversationTitleInTabNames::hierarchy(),
        Some("appearance.vertical_tabs")
    );
    assert_eq!(
        UseLatestUserPromptAsConversationTitleInTabNames::toml_key(),
        "use_latest_prompt_as_title"
    );
}

#[test]
fn show_vertical_tab_panel_in_restored_windows_defaults_to_false() {
    App::test((), |mut app| async move {
        initialize_settings_for_tests(&mut app);

        TabSettings::handle(&app).read(&app, |settings, _ctx| {
            assert!(!*settings.show_vertical_tab_panel_in_restored_windows);
        });
    });
}

#[test]
fn show_vertical_tab_panel_in_restored_windows_uses_vertical_tabs_path() {
    assert_eq!(
        ShowVerticalTabPanelInRestoredWindows::toml_path(),
        Some("appearance.vertical_tabs.show_panel_in_restored_windows")
    );
    assert_eq!(
        ShowVerticalTabPanelInRestoredWindows::hierarchy(),
        Some("appearance.vertical_tabs")
    );
    assert_eq!(
        ShowVerticalTabPanelInRestoredWindows::toml_key(),
        "show_panel_in_restored_windows"
    );
}

#[test]
fn hide_title_bar_search_bar_in_vertical_tabs_defaults_to_false() {
    App::test((), |mut app| async move {
        initialize_settings_for_tests(&mut app);

        TabSettings::handle(&app).read(&app, |settings, _ctx| {
            assert!(!*settings.hide_title_bar_search_bar_in_vertical_tabs);
        });
    });
}

#[test]
fn hide_title_bar_search_bar_in_vertical_tabs_uses_vertical_tabs_path() {
    assert_eq!(
        HideTitleBarSearchBarInVerticalTabs::toml_path(),
        Some("appearance.vertical_tabs.hide_title_bar_search_bar")
    );
    assert_eq!(
        HideTitleBarSearchBarInVerticalTabs::hierarchy(),
        Some("appearance.vertical_tabs")
    );
    assert_eq!(
        HideTitleBarSearchBarInVerticalTabs::toml_key(),
        "hide_title_bar_search_bar"
    );
}

#[test]
fn header_toolbar_chip_selection_default_contains_code_review() {
    let config = HeaderToolbarChipSelection::Default;
    assert!(config.contains_item(&HeaderToolbarItemKind::CodeReview));
}

#[test]
fn header_toolbar_chip_selection_custom_without_code_review_reports_absent() {
    let config = HeaderToolbarChipSelection::Custom {
        left: vec![
            HeaderToolbarItemKind::TabsPanel,
            HeaderToolbarItemKind::ToolsPanel,
        ],
        right: vec![HeaderToolbarItemKind::NotificationsMailbox],
    };
    assert!(!config.contains_item(&HeaderToolbarItemKind::CodeReview));
    assert!(config.contains_item(&HeaderToolbarItemKind::TabsPanel));
    assert!(config.contains_item(&HeaderToolbarItemKind::ToolsPanel));
    assert!(config.contains_item(&HeaderToolbarItemKind::NotificationsMailbox));
    assert!(!config.contains_item(&HeaderToolbarItemKind::AgentManagement));
}

#[test]
fn header_toolbar_chip_selection_custom_with_code_review_on_left_reports_present() {
    let config = HeaderToolbarChipSelection::Custom {
        left: vec![HeaderToolbarItemKind::CodeReview],
        right: vec![],
    };
    assert!(config.contains_item(&HeaderToolbarItemKind::CodeReview));
}

#[test]
fn header_toolbar_chip_selection_custom_empty_reports_all_absent() {
    let config = HeaderToolbarChipSelection::Custom {
        left: vec![],
        right: vec![],
    };
    for item in HeaderToolbarItemKind::all_items() {
        assert!(!config.contains_item(&item));
    }
}

#[test]
#[cfg(feature = "warp_lite")]
fn lite_sidebar_migrates_old_horizontal_layout_once() {
    use warp_core::user_preferences::GetUserPreferences;

    App::test((), |mut app| async move {
        initialize_settings_for_tests(&mut app);
        app.update(|ctx| {
            ctx.private_user_preferences()
                .remove_value(LITE_SIDEBAR_MIGRATION_KEY)
                .unwrap();
            TabSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings.use_vertical_tabs.set_value(false, ctx).unwrap();
                settings
                    .header_toolbar_chip_selection
                    .set_value(
                        HeaderToolbarChipSelection::Custom {
                            left: vec![],
                            right: vec![HeaderToolbarItemKind::TabsPanel],
                        },
                        ctx,
                    )
                    .unwrap();
            });
            migrate_lite_sidebar_settings(ctx);
            let settings = TabSettings::as_ref(ctx);
            assert!(*settings.use_vertical_tabs);
            assert!(*settings.show_vertical_tab_panel_in_restored_windows);
            assert_eq!(
                settings.vertical_tabs_primary_info,
                VerticalTabsPrimaryInfo::WorkingDirectory
            );
            assert_eq!(
                settings.vertical_tabs_compact_subtitle,
                VerticalTabsCompactSubtitle::Command
            );
            assert_eq!(
                settings.header_toolbar_chip_selection.left_items(),
                vec![HeaderToolbarItemKind::TabsPanel]
            );
            assert!(
                settings
                    .header_toolbar_chip_selection
                    .right_items()
                    .is_empty()
            );
            assert_eq!(
                ctx.private_user_preferences()
                    .read_value(LITE_SIDEBAR_MIGRATION_KEY)
                    .unwrap()
                    .as_deref(),
                Some("true")
            );

            TabSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings.use_vertical_tabs.set_value(false, ctx).unwrap();
                settings
                    .vertical_tabs_primary_info
                    .set_value(VerticalTabsPrimaryInfo::Command, ctx)
                    .unwrap();
            });
            migrate_lite_sidebar_settings(ctx);
            let settings = TabSettings::as_ref(ctx);
            assert!(!*settings.use_vertical_tabs);
            assert_eq!(
                settings.vertical_tabs_primary_info,
                VerticalTabsPrimaryInfo::Command
            );
        });
    });
}
