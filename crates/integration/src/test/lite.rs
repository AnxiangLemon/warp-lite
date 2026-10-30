#[cfg(feature = "warp_lite")]
#[path = "lite_backgrounds.rs"]
mod backgrounds;
#[cfg(feature = "warp_lite")]
#[path = "lite_close_confirmation.rs"]
mod close_confirmation;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[cfg(feature = "warp_lite")]
pub use backgrounds::{
    test_lite_corrupt_background, test_lite_fixed_background, test_lite_jpeg_background,
    test_lite_missing_background, test_lite_oversized_background,
};
#[cfg(feature = "warp_lite")]
pub use close_confirmation::test_lite_close_confirmation;
use diesel::connection::SimpleConnection as _;
use diesel::sql_types::{Binary, Nullable, Text};
use diesel::sqlite::SqliteConnection;
use diesel::{Connection as _, QueryableByName, RunQueryDsl as _, sql_query};
use settings::Setting as _;
use warp::appearance::Appearance;
use warp::integration_testing::assertions::{
    assert_lite_runtime_services_are_absent, assert_logged_out,
};
use warp::integration_testing::command_palette::{close_command_palette, open_command_palette};
use warp::integration_testing::command_search::{
    assert_command_search_has_results, assert_command_search_is_open,
    assert_history_filter_is_active,
};
use warp::integration_testing::context_chips::{
    DisplayChip, DisplayChipAction, DisplayChipKind, DisplayChipMenu, DisplayChipMenuAction,
};
use warp::integration_testing::pane_group::{assert_focused_pane_index, assert_num_panes_in_tab};
use warp::integration_testing::persistence::{
    PersistenceScope, database_file_path_for_current_scope, database_file_path_for_scope,
};
use warp::integration_testing::step::new_step_with_default_assertions;
use warp::integration_testing::tab::assert_tab_title;
use warp::integration_testing::terminal::util::{ExactLine, ExpectedExitStatus};
use warp::integration_testing::terminal::{
    assert_active_block_output_for_single_terminal_in_tab, assert_active_block_received_precmd,
    assert_input_is_focused, assert_long_running_block_executing_for_single_terminal_in_tab,
    assert_no_block_executing, execute_command_for_single_terminal_in_tab,
    wait_until_bootstrapped_pane, wait_until_bootstrapped_single_pane_for_tab,
};
use warp::integration_testing::view_getters::{
    pane_group_view, single_terminal_view_for_tab, workspace_view,
};
use warp::integration_testing::workspace::{
    AddTabWithShellSource, assert_focused_tab_index, assert_tab_count,
};
use warp::settings::{AISettings, DebugSettings, RESTORE_SESSION, ThemeSettings};
use warp::settings_view::{SettingsSection, SettingsView};
use warp::terminal::available_shells::AvailableShells;
use warp::themes::theme::ThemeKind;
use warp::themes::theme_chooser::ThemeChooserMode;
use warp::workspace::WorkspaceAction;
use warp::workspace::header_toolbar_item::HeaderToolbarItemKind;
use warp::workspace::tab_settings::{TabSettings, VerticalTabsDisplayGranularity};
use warpui_core::assets::asset_cache::{AssetCache, AssetSource, AssetState};
use warpui_core::elements::DEFAULT_UI_LINE_HEIGHT_RATIO;
use warpui_core::image_cache::ImageType;
use warpui_core::integration::{ARTIFACTS_DIR_ENV_VAR, AssertionOutcome, TestSetupUtils, TestStep};
use warpui_core::platform::LineStyle;
use warpui_core::text_layout::{DEFAULT_TOP_BOTTOM_RATIO, StyleAndFont, TextStyle};
use warpui_core::{SingletonEntity as _, async_assert, async_assert_eq};

use super::TEST_ONLY_ASSETS;
use crate::Builder;

const TAB_TITLE: &str = "Lite project";
const OUTPUT_MARKER: &str = "warp-lite-native-shell-output";
const CANCEL_READY_MARKER: &str = "warp-lite-cancel-ready";
const DIRECTORY_ENV: &str = "WARP_LITE_SMOKE_DIRECTORY";
const STATE_DIR_ENV: &str = "WARP_LITE_TEST_STATE_DIR";
const DATABASE_ENV: &str = "WARP_LITE_TEST_DATABASE";

#[derive(QueryableByName)]
struct SavedTab {
    #[diesel(sql_type = Nullable<Text>)]
    custom_title: Option<String>,
}

#[derive(QueryableByName)]
struct SavedGroup {
    #[diesel(sql_type = Nullable<Text>)]
    name: Option<String>,
}

#[derive(QueryableByName)]
struct SavedDirectory {
    #[diesel(sql_type = Nullable<Text>)]
    cwd: Option<String>,
}

#[derive(QueryableByName)]
struct SavedBlock {
    #[diesel(sql_type = Nullable<Text>)]
    pwd: Option<String>,
    #[diesel(sql_type = Binary)]
    stylized_output: Vec<u8>,
}

fn lite_builder() -> Builder {
    Builder::new()
        .set_should_run_test(|| cfg!(target_os = "windows") && warp::features::IS_LITE_BUILD)
        .with_user_defaults(HashMap::from([(
            RESTORE_SESSION.to_owned(),
            "true".to_owned(),
        )]))
}

fn prepare_directory(utils: &mut TestSetupUtils) -> PathBuf {
    let state_dir = warp_core::paths::state_dir();
    let database_path = database_file_path_for_scope(&PersistenceScope::App);
    assert_test_namespace(&state_dir);
    assert_test_namespace(&database_path);
    assert_eq!(database_path.parent(), Some(state_dir.as_path()));
    assert_eq!(
        database_path.file_name(),
        Some(std::ffi::OsStr::new("warp.sqlite"))
    );
    utils.set_env(STATE_DIR_ENV, Some(state_dir));
    utils.set_env(DATABASE_ENV, Some(database_path));
    let shell_path = PathBuf::from(
        std::env::var_os("SystemRoot").expect("Windows should expose its system directory"),
    )
    .join("System32/WindowsPowerShell/v1.0/powershell.exe");
    assert!(
        shell_path.is_file(),
        "Native PowerShell should be installed at {shell_path:?}"
    );
    utils.set_env("WARP_SHELL_PATH", Some(shell_path));
    let directory = utils.test_dir().join("Lite 项目 with spaces");
    std::fs::create_dir_all(&directory).expect("should create the test project directory");
    utils.set_env(DIRECTORY_ENV, Some(&directory));
    utils.set_env("HOME", Some(utils.test_dir()));
    utils.set_env("USERPROFILE", Some(utils.test_dir()));
    if std::env::var_os(ARTIFACTS_DIR_ENV_VAR).is_none() {
        utils.set_env(
            ARTIFACTS_DIR_ENV_VAR,
            Some(test_state_dir().join("artifacts")),
        );
    }
    directory
}

fn assert_test_namespace(path: &Path) {
    assert!(
        path.is_absolute()
            && !path
                .components()
                .any(|component| component == std::path::Component::ParentDir)
            && path.components().any(|component| {
                component
                    .as_os_str()
                    .to_string_lossy()
                    .starts_with("WarpIntegrationLiteSmoke-")
            }),
        "Lite test paths must stay in an absolute, dedicated test namespace: {path:?}"
    );
}

fn test_state_dir() -> PathBuf {
    let path = PathBuf::from(
        std::env::var_os(STATE_DIR_ENV).expect("test state path should be captured during setup"),
    );
    assert_test_namespace(&path);
    path
}

fn test_database_path() -> PathBuf {
    let path = PathBuf::from(
        std::env::var_os(DATABASE_ENV).expect("test database path should be captured during setup"),
    );
    assert_test_namespace(&path);
    assert_eq!(path.parent(), Some(test_state_dir().as_path()));
    assert_eq!(path.file_name(), Some(std::ffi::OsStr::new("warp.sqlite")));
    path
}

fn same_directory(actual: &str, expected: &Path) -> bool {
    Path::new(actual).canonicalize().ok() == expected.canonicalize().ok() && expected.is_dir()
}

fn searchable_directory_chip(
    app: &warpui_core::App,
    window_id: warpui_core::WindowId,
) -> Option<warpui_core::ViewHandle<DisplayChip>> {
    app.views_of_type::<DisplayChip>(window_id)?
        .into_iter()
        .find(|chip| {
            chip.read(app, |chip, ctx| match chip.display_chip_kind() {
                DisplayChipKind::WorkingDirectory {
                    directory_fetcher, ..
                } => directory_fetcher
                    .as_ref(ctx)
                    .cached_files()
                    .iter()
                    .any(|item| item.name == "搜索测试 目录"),
                _ => false,
            })
        })
}

fn searchable_directory_menu(
    app: &warpui_core::App,
    window_id: warpui_core::WindowId,
) -> Option<warpui_core::ViewHandle<DisplayChipMenu>> {
    searchable_directory_chip(app, window_id)?.read(app, |chip, _| match chip.display_chip_kind() {
        DisplayChipKind::WorkingDirectory { menu, .. } => Some(menu.clone()),
        _ => None,
    })
}

pub fn test_lite_chinese_settings() -> Builder {
    let mut builder = lite_builder()
        .with_user_defaults(HashMap::from([(
            "ShowMemoryStats".to_owned(),
            "false".to_owned(),
        )]))
        .with_setup(|utils| {
            let directory = prepare_directory(utils);
            std::fs::create_dir_all(directory.join("搜索测试 目录")).unwrap();
            std::fs::write(directory.join("readme.txt"), "directory search fixture").unwrap();
        })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(execute_command_for_single_terminal_in_tab(
            0,
            format!("Set-Location -LiteralPath $env:{DIRECTORY_ENV}"),
            ExpectedExitStatus::Success,
            (),
        ))
        .with_step(
            new_step_with_default_assertions("Directory chip reads local files and folders")
                .add_named_assertion(
                    "The directory menu contains the fixture",
                    |app, window_id| {
                        async_assert!(
                            searchable_directory_chip(app, window_id).is_some(),
                            "Directory chip must list the current directory"
                        )
                    },
                ),
        )
        .with_step(
            new_step_with_default_assertions("Open the directory menu").with_action(
                |app, window_id, _| {
                    let chip = searchable_directory_chip(app, window_id).unwrap();
                    app.dispatch_typed_action(
                        window_id,
                        &[chip.id()],
                        &DisplayChipAction::ToggleMenu,
                    );
                },
            ),
        )
        .with_step(
            new_step_with_default_assertions("Filter the directory menu with real text input")
                .with_typed_characters(&["搜索测试"])
                .add_named_assertion("Typing Chinese filters to one folder", |app, window_id| {
                    let menu = searchable_directory_menu(app, window_id).unwrap();
                    menu.read(app, |menu, _| {
                        async_assert_eq!(
                            menu.filtered_item_names(),
                            vec!["搜索测试 目录".to_owned()]
                        )
                    })
                })
                .with_take_screenshot("lite-directory-search.png"),
        )
        .with_step(
            new_step_with_default_assertions("Select the filtered directory")
                .with_action(|app, window_id, _| {
                    let menu = searchable_directory_menu(app, window_id).unwrap();
                    app.dispatch_typed_action(
                        window_id,
                        &[menu.id()],
                        &DisplayChipMenuAction::SelectEnter,
                    );
                })
                .add_named_assertion(
                    "Selection changes the actual shell directory",
                    |app, window_id| {
                        let expected = PathBuf::from(std::env::var_os(DIRECTORY_ENV).unwrap())
                            .join("搜索测试 目录");
                        single_terminal_view_for_tab(app, window_id, 0).read(app, |view, _| {
                            async_assert!(
                                view.pwd()
                                    .as_deref()
                                    .is_some_and(|pwd| same_directory(pwd, &expected)),
                                "Selecting a search result must change the working directory"
                            )
                        })
                    },
                ),
        )
        .with_step(assert_logged_out())
        .with_step(assert_lite_runtime_services_are_absent())
        .with_step(
            new_step_with_default_assertions(
                "Verify the local terminal before opening Lite settings",
            )
            .add_named_assertion(
                "The terminal runs locally with AI and debug overlays disabled",
                |app, window_id| {
                    single_terminal_view_for_tab(app, window_id, 0).read(app, |view, ctx| {
                        async_assert!(
                            view.active_session_is_local(ctx) == Some(true)
                                && !AISettings::as_ref(ctx).is_any_ai_enabled(ctx)
                                && !DebugSettings::as_ref(ctx).should_show_memory_stats(),
                            "Lite should have a local terminal without AI or debug overlays"
                        )
                    })
                },
            )
            .add_named_assertion(
                "The terminal window has usable dimensions",
                |app, window_id| {
                    let bounds = app.window_bounds(&window_id);
                    async_assert!(
                            bounds.is_some_and(
                                |bounds| bounds.width() >= 640. && bounds.height() >= 360.
                            ),
                            "Expected a usable terminal window, got {bounds:?}"
                        )
                },
            )
            .add_named_assertion(
                "System CJK fallback shapes Chinese interface text without missing glyphs",
                |app, _| {
                    app.read(|ctx| {
                        let appearance = Appearance::as_ref(ctx);
                        let sample = "终端设置，字体与字号；复制粘贴。";
                        let line = ctx.font_cache().text_layout_system().layout_line(
                            sample,
                            LineStyle {
                                font_size: appearance.ui_font_size(),
                                line_height_ratio: DEFAULT_UI_LINE_HEIGHT_RATIO,
                                baseline_ratio: DEFAULT_TOP_BOTTOM_RATIO,
                                fixed_width_tab_size: None,
                            },
                            &[(
                                0..sample.chars().count(),
                                StyleAndFont::new(
                                    appearance.ui_font_family(),
                                    Default::default(),
                                    TextStyle::new(),
                                ),
                            )],
                            1000.,
                            Default::default(),
                        );
                        async_assert!(
                            line.chars_with_missing_glyphs.is_empty()
                                && line
                                    .runs
                                    .iter()
                                    .flat_map(|run| &run.glyphs)
                                    .all(|glyph| glyph.id != 0),
                            "Chinese interface text is missing glyphs: {:?}",
                            line.chars_with_missing_glyphs
                        )
                    })
                },
            ),
        );

    for (section, screenshot) in [
        (SettingsSection::Features, "lite-settings-terminal.png"),
        (SettingsSection::Appearance, "lite-settings-appearance.png"),
        (SettingsSection::About, "lite-settings-about.png"),
        (
            SettingsSection::Keybindings,
            "lite-settings-keybindings.png",
        ),
    ] {
        let mut step = new_step_with_default_assertions(&format!("Open the simplified Lite {section:?} settings page"))
            .with_action(move |app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(
                    window_id,
                    &[workspace.id()],
                    &WorkspaceAction::ShowSettingsPage(section),
                );
            })
            .add_named_assertion("The selected settings page is visible and only local settings are available", move |app, window_id| {
                let settings = app.views_of_type::<SettingsView>(window_id)
                    .and_then(|views| views.into_iter().next());
                let Some(settings) = settings else {
                    return AssertionOutcome::failure("Settings view has not opened yet".to_owned());
                };
                settings.read(app, |view, ctx| {
                    let actual_section = view.current_settings_section();
                    let visible_sections = view.visible_sections_for_integration_test(ctx);
                    async_assert!(
                        actual_section == section && visible_sections == vec![
                            SettingsSection::Appearance,
                            SettingsSection::Features,
                            SettingsSection::Keybindings,
                            SettingsSection::About,
                        ],
                        "Expected local settings page {section:?}, got current={actual_section:?}, visible={visible_sections:?}"
                    )
                })
            })
            .add_named_assertion("Settings reuse one top-level entry at the bottom", |app, window_id| {
                workspace_view(app, window_id).read(app, |workspace, ctx| {
                    let snapshot = workspace.snapshot(window_id, false, ctx);
                    if snapshot.tabs.len() != 2 {
                        return AssertionOutcome::failure("Opening settings repeatedly must reuse the same tab".to_owned());
                    }
                    async_assert!(snapshot.tabs[1].group_id.is_none() && snapshot.tabs[1].custom_title.as_deref() == Some("设置"), "Settings must be an ungrouped final tab")
                })
            })
            .add_named_assertion("The unavailable project and code tool panels stay hidden", |app, window_id| {
                workspace_view(app, window_id).read(app, |workspace, ctx| {
                    async_assert!(
                        !workspace.is_left_panel_open(ctx)
                            && !HeaderToolbarItemKind::ToolsPanel.is_available(ctx)
                            && !HeaderToolbarItemKind::CodeReview.is_available(ctx),
                        "Lite settings must not reopen project, global search, or code tool panels"
                    )
                })
            });
        if section == SettingsSection::About {
            step =
                step.add_named_assertion("The bundled Lite logo decodes successfully", |app, _| {
                    app.read(|ctx| {
                        match AssetCache::as_ref(ctx).load_asset::<ImageType>(
                            AssetSource::Bundled {
                                path: "bundled/svg/warp-lite-logo.svg",
                            },
                        ) {
                            AssetState::Loaded { .. } => AssertionOutcome::Success,
                            AssetState::FailedToLoad(error) => AssertionOutcome::failure(format!(
                                "Lite logo failed to load: {error:#}"
                            )),
                            AssetState::Loading { .. } | AssetState::Evicted => {
                                AssertionOutcome::failure("Lite logo is not ready".to_owned())
                            }
                        }
                    })
                });
        }
        if std::env::var_os("WARPUI_USE_REAL_DISPLAY_IN_INTEGRATION_TESTS").is_some() {
            step = step.with_take_screenshot(screenshot);
        }
        builder = builder.with_step(step);
    }

    for (query, expected) in [
        ("字号", vec![SettingsSection::Appearance]),
        ("智能", vec![SettingsSection::Features]),
        ("没有这项设置", vec![]),
        (
            "",
            vec![
                SettingsSection::Appearance,
                SettingsSection::Features,
                SettingsSection::Keybindings,
                SettingsSection::About,
            ],
        ),
    ] {
        builder = builder.with_step(
            new_step_with_default_assertions(&format!("Filter settings with {query:?}"))
                .with_action(move |app, window_id, _| {
                    let settings = app.views_of_type::<SettingsView>(window_id).unwrap()[0].clone();
                    settings.update(app, |view, ctx| view.set_search_query(query, ctx));
                })
                .add_named_assertion(
                    "Chinese search filters settings and clearing restores all pages",
                    move |app, window_id| {
                        let settings =
                            app.views_of_type::<SettingsView>(window_id).unwrap()[0].clone();
                        settings.read(app, |view, ctx| {
                            async_assert_eq!(
                                view.visible_sections_for_integration_test(ctx),
                                expected
                            )
                        })
                    },
                ),
        );
    }
    builder = builder
        .with_step(
            new_step_with_default_assertions("Open the Chinese theme chooser")
                .with_action(|app, window_id, _| {
                    let workspace = workspace_view(app, window_id);
                    app.dispatch_typed_action(
                        window_id,
                        &[workspace.id()],
                        &WorkspaceAction::ShowThemeChooser(ThemeChooserMode::SystemAgnostic),
                    );
                })
                .with_take_screenshot("lite-theme-chooser.png"),
        )
        .with_step(
            new_step_with_default_assertions("Choose a theme by its Chinese name")
                .with_typed_characters(&["紫莓"])
                .with_keystrokes(&["down"])
                .add_named_assertion(
                    "The Chinese result selects the original theme identifier",
                    |app, _| {
                        app.read(|ctx| {
                            async_assert_eq!(
                                *ThemeSettings::as_ref(ctx).theme_kind.value(),
                                ThemeKind::Adeberry
                            )
                        })
                    },
                )
                .with_take_screenshot("lite-theme-search-chinese.png"),
        )
        .with_step(
            new_step_with_default_assertions("Close the theme chooser")
                .with_keystrokes(&["escape"]),
        );
    builder.with_step(
        new_step_with_default_assertions("Open a terminal while settings is active")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(
                    window_id,
                    &[workspace.id()],
                    &WorkspaceAction::AddTerminalTab {
                        hide_homepage: true,
                    },
                );
            })
            .add_named_assertion(
                "Settings stays last when opening a terminal",
                |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, ctx| {
                        let snapshot = workspace.snapshot(window_id, false, ctx);
                        async_assert!(
                            snapshot.tabs.len() == 3
                                && snapshot.tabs[2].group_id.is_none()
                                && snapshot.tabs[2].custom_title.as_deref() == Some("设置"),
                            "Settings must remain ungrouped at the end"
                        )
                    })
                },
            ),
    )
}

pub fn test_lite_terminal_management() -> Builder {
    lite_builder()
        .with_user_defaults(HashMap::from([(
            "UseVerticalTabs".to_owned(),
            "false".to_owned(),
        )]))
        .with_real_display()
        .with_setup(|utils| {
            let directory = prepare_directory(utils);
            std::fs::create_dir_all(directory.join("另一个项目"))
                .expect("should create a second project directory");
        })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(assert_lite_runtime_services_are_absent())
        .with_step(
            new_step_with_default_assertions("Migrate the old horizontal layout into a visible sidebar")
                .add_named_assertion("The old preference now opens the vertical tab sidebar", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, ctx| {
                        async_assert!(
                            *TabSettings::as_ref(ctx).use_vertical_tabs
                                && workspace.snapshot(window_id, false, ctx).vertical_tabs_panel_open,
                            "Lite should migrate an old horizontal layout into an open sidebar"
                        )
                    })
                }),
        )
        .with_step(execute_command_for_single_terminal_in_tab(
            0,
            format!("Set-Location -LiteralPath $env:{DIRECTORY_ENV}"),
            ExpectedExitStatus::Success,
            (),
        ))
        .with_step(
            new_step_with_default_assertions("Render ordinary shell icons in vertical tabs")
                .with_action(|app, _, _| {
                    TabSettings::handle(app).update(app, |settings, ctx| {
                        settings
                            .use_vertical_tabs
                            .set_value(true, ctx)
                            .expect("enable vertical tabs");
                        settings
                            .vertical_tabs_display_granularity
                            .set_value(VerticalTabsDisplayGranularity::Panes, ctx)
                            .expect("show terminal pane rows");
                    });
                })
                .add_named_assertion("The local shell row was rendered", |app, window_id| {
                    let group = pane_group_view(app, window_id, 0);
                    let row = group.read(app, |pane_group, ctx| {
                        format!(
                            "vertical_tabs:pane_row:{:?}:{}",
                            group.id(),
                            pane_group.focused_pane_id(ctx)
                        )
                    });
                    app.read(|ctx| {
                        async_assert!(
                            ctx.element_position_by_id_at_last_frame(window_id, row)
                                .is_some()
                        )
                    })
                }),
        )
        .with_step(
            new_step_with_default_assertions("Create and focus a second local terminal tab")
                .with_action(|app, window_id, _| {
                    let workspace = workspace_view(app, window_id);
                    app.dispatch_typed_action(
                        window_id,
                        &[workspace.id()],
                        &WorkspaceAction::AddTerminalTab {
                            hide_homepage: true,
                        },
                    );
                })
                .add_named_assertion("Two tabs are visible", assert_tab_count(2))
                .add_named_assertion("The new tab is focused", assert_focused_tab_index(1)),
        )
        .with_step(wait_until_bootstrapped_single_pane_for_tab(1))
        .with_step(execute_command_for_single_terminal_in_tab(
            1,
            format!("Set-Location -LiteralPath (Join-Path $env:{DIRECTORY_ENV} '另一个项目')"),
            ExpectedExitStatus::Success,
            (),
        ))
        .with_step(
            new_step_with_default_assertions("Group the two project terminals in the sidebar")
                .with_action(|app, window_id, _| {
                    let workspace = workspace_view(app, window_id);
                    for tab_index in [0, 1] {
                        app.dispatch_typed_action(
                            window_id,
                            &[workspace.id()],
                            &WorkspaceAction::NewTabGroupFromTab(tab_index),
                        );
                    }
                })
                .add_named_assertion("Two groups are stacked on the left of the terminal", |app, window_id| {
                    let snapshot = workspace_view(app, window_id)
                        .read(app, |workspace, ctx| workspace.snapshot(window_id, false, ctx));
                    if snapshot.tab_groups.len() != 2 {
                        return AssertionOutcome::failure(format!(
                            "Expected two project groups, got {}", snapshot.tab_groups.len()
                        ));
                    }
                    let first = snapshot.tabs[0].group_id.expect("first tab is grouped");
                    let second = snapshot.tabs[1].group_id.expect("second tab is grouped");
                    let names = workspace_view(app, window_id).read(app, |workspace, ctx| {
                        (workspace.tab_group_display_name(first, ctx), workspace.tab_group_display_name(second, ctx))
                    });
                    if names != ("Lite 项目 with spaces".to_owned(), "另一个项目".to_owned()) {
                        return AssertionOutcome::failure(format!("Expected the two directory names, got {names:?}"));
                    }
                    let terminal_id = single_terminal_view_for_tab(app, window_id, 1)
                        .read(app, |view, _| view.terminal_position_id());
                    app.read(|ctx| {
                        let first = ctx.element_position_by_id_at_last_frame(
                            window_id, format!("vertical_tabs:group:{first:?}"),
                        );
                        let second = ctx.element_position_by_id_at_last_frame(
                            window_id, format!("vertical_tabs:group:{second:?}"),
                        );
                        let terminal = ctx.element_position_by_id_at_last_frame(window_id, terminal_id);
                        async_assert!(
                            matches!((first, second, terminal), (Some(a), Some(b), Some(t))
                                if a.width() >= 180. && (a.min_x() - b.min_x()).abs() < 1.
                                    && b.min_y() >= a.max_y() && b.max_x() <= t.min_x()),
                            "Expected vertically stacked left-side groups, got first={first:?}, second={second:?}, terminal={terminal:?}"
                        )
                    })
                })
                .with_take_screenshot("lite-directory-groups.png"),
        )
        .with_step(
            new_step_with_default_assertions("Distinguish a collapsed group from terminal rows")
                .with_action(|app, window_id, _| {
                    let workspace = workspace_view(app, window_id);
                    let group_id = workspace.read(app, |workspace, ctx| {
                        workspace.snapshot(window_id, false, ctx).tabs[0].group_id.unwrap()
                    });
                    app.dispatch_typed_action(window_id, &[workspace.id()],
                        &WorkspaceAction::ToggleTabGroupCollapsed(group_id));
                })
                .with_take_screenshot("lite-collapsed-group.png"),
        )
        .with_step(
            new_step_with_default_assertions("Rename a group while keeping the other directory title automatic")
                .with_action(|app, window_id, _| {
                    let workspace = workspace_view(app, window_id);
                    let group_id = workspace.read(app, |workspace, ctx| {
                        workspace.snapshot(window_id, false, ctx).tabs[1].group_id
                            .expect("second tab should be grouped")
                    });
                    app.dispatch_typed_action(
                        window_id, &[workspace.id()], &WorkspaceAction::RenameTabGroup(group_id),
                    );
                }),
        )
        .with_step(
            new_step_with_default_assertions("Keep a custom Chinese group name")
                .with_typed_characters(&["工具终端"])
                .with_keystrokes(&["enter"])
                .add_named_assertion("Custom and automatic group names are saved independently", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, ctx| {
                        let snapshot = workspace.snapshot(window_id, false, ctx);
                        let first = snapshot.tabs[0].group_id;
                        let second = snapshot.tabs[1].group_id;
                        async_assert!(
                            snapshot.tab_groups.iter().any(|group| Some(group.id) == first && group.name.is_none())
                                && snapshot.tab_groups.iter().any(|group| Some(group.id) == second && group.name.as_deref() == Some("工具终端")),
                            "Expected an automatic directory title and the custom name 工具终端"
                        )
                    })
                })
                .with_take_screenshot("lite-custom-group-name.png"),
        )
        .with_step(
            new_step_with_default_assertions("Close the new tab and return to the original")
                .with_action(|app, window_id, _| {
                    let workspace = workspace_view(app, window_id);
                    app.dispatch_typed_action(
                        window_id,
                        &[workspace.id()],
                        &WorkspaceAction::CloseActiveTab,
                    );
                })
                .add_named_assertion("Closing a tab does not show a quit dialog", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, ctx| async_assert!(!workspace.is_native_quit_modal_open(ctx), "Ordinary tabs should close directly"))
                })
                .add_named_assertion("One tab remains", assert_tab_count(1))
                .add_named_assertion("The original tab is focused", assert_focused_tab_index(0)),
        )
        .with_step(
            new_step_with_default_assertions("Split the terminal and focus the new pane")
                .with_keystrokes(&["ctrl-shift-D"])
                .add_named_assertion(
                    "Two terminal panes are visible",
                    assert_num_panes_in_tab(0, 2),
                )
                .add_named_assertion("The new pane is focused", assert_focused_pane_index(0, 1)),
        )
        .with_step(wait_until_bootstrapped_pane(0, 1))
        .with_step(
            new_step_with_default_assertions("Request closing the split without removing it prematurely")
                .with_keystrokes(&["ctrl-shift-W"])
                .add_named_assertion("Both panes remain until the close is confirmed", assert_num_panes_in_tab(0, 2))
                .add_named_assertion("Closing an idle pane requires confirmation", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, ctx| async_assert!(workspace.is_close_session_confirmation_dialog_open(ctx), "The pane close confirmation dialog is missing"))
                }),
        )
        .with_step(
            new_step_with_default_assertions("Cancel the split close with a real Escape key")
                .with_keystrokes(&["escape"])
                .add_named_assertion("Cancelling keeps both terminal panes", assert_num_panes_in_tab(0, 2))
                .add_named_assertion("Cancelling keeps the selected pane", assert_focused_pane_index(0, 1))
                .add_named_assertion("Escape dismisses the pane close confirmation", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, ctx| async_assert!(!workspace.is_close_session_confirmation_dialog_open(ctx), "The pane close dialog did not dismiss"))
                }),
        )
        .with_step(
            new_step_with_default_assertions("Request closing the same split again after cancelling")
                .with_keystrokes(&["ctrl-shift-W"])
                .add_named_assertion("The renewed pane close again requires confirmation", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, ctx| async_assert!(workspace.is_close_session_confirmation_dialog_open(ctx), "The second pane close request must also be confirmed"))
                })
                .add_named_assertion("Both panes remain until the renewed request is confirmed", assert_num_panes_in_tab(0, 2))
                .with_take_screenshot("lite-confirm-pane-close.png"),
        )
        .with_step(
            new_step_with_default_assertions("Confirm the split close through the real dialog button")
                .with_click_on_saved_position("close-session-confirm")
                .add_named_assertion("The original tab remains", assert_tab_count(1))
                .add_named_assertion("One terminal pane remains", assert_num_panes_in_tab(0, 1))
                .add_named_assertion(
                    "The original pane is focused",
                    assert_focused_pane_index(0, 0),
                )
                .add_named_assertion("The terminal input owns focus", assert_input_is_focused()),
        )
        .with_step(assert_lite_runtime_services_are_absent())
}

pub fn test_lite_terminal_smoke() -> Builder {
    let builder = lite_builder()
        .with_setup(|utils| {
            prepare_directory(utils);
        })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(assert_logged_out())
        .with_step(
            new_step_with_default_assertions("Create a second local terminal tab")
                .with_action(|app, window_id, _| {
                    workspace_view(app, window_id).update(app, |workspace, ctx| {
                        workspace.add_terminal_tab(true, ctx);
                    });
                })
                .add_named_assertion("Two terminal tabs are open", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, _| {
                        async_assert_eq!(workspace.tab_count(), 2)
                    })
                }),
        )
        .with_step(wait_until_bootstrapped_single_pane_for_tab(1))
        .with_step(
            new_step_with_default_assertions("Name the project tab")
                .with_action(|app, window_id, _| {
                    pane_group_view(app, window_id, 1).update(app, |pane_group, ctx| {
                        pane_group.set_title(TAB_TITLE, ctx);
                    });
                })
                .add_named_assertion("Project tab displays its custom name", assert_tab_title(1, TAB_TITLE)),
        )
        .with_step(execute_command_for_single_terminal_in_tab(
            1,
            format!("Set-Location -LiteralPath $env:{DIRECTORY_ENV}"),
            ExpectedExitStatus::Success,
            (),
        ))
        .with_step(execute_command_for_single_terminal_in_tab(
            1,
            "git init --quiet; if ($LASTEXITCODE -ne 0) { throw 'Git initialization failed' }; git rev-parse --is-inside-work-tree".to_owned(),
            ExpectedExitStatus::Success,
            ExactLine::from("true"),
        ))
        .with_step(execute_command_for_single_terminal_in_tab(
            1,
            format!("Write-Output '{OUTPUT_MARKER}'"),
            ExpectedExitStatus::Success,
            OUTPUT_MARKER.to_owned(),
        ))
        .with_step(execute_command_for_single_terminal_in_tab(
            1,
            "Write-Output '终端中文验证'".to_owned(),
            ExpectedExitStatus::Success,
            "终端中文验证".to_owned(),
        ))
        .with_step(
            new_step_with_default_assertions("Save the project terminal in an automatic directory group")
                .with_action(|app, window_id, _| {
                    let workspace = workspace_view(app, window_id);
                    app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::NewTabGroupFromTab(1));
                })
                .add_named_assertion("The project group displays its directory name", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, ctx| {
                        let snapshot = workspace.snapshot(window_id, false, ctx);
                        let Some(group_id) = snapshot.tabs[1].group_id else {
                            return AssertionOutcome::failure("The project tab should be grouped".to_owned());
                        };
                        async_assert_eq!(workspace.tab_group_display_name(group_id, ctx), "Lite 项目 with spaces")
                    })
                }),
        )
        .with_step(
            TestStep::new("Start a cancellable native shell command")
                .with_typed_characters(&[&format!(
                    "Write-Output '{CANCEL_READY_MARKER}'; Start-Sleep -Seconds 30"
                )])
                .with_keystrokes(&["enter"])
                .add_named_assertion(
                    "Cancellable command has entered shell execution",
                    assert_long_running_block_executing_for_single_terminal_in_tab(true, 1),
                )
                .add_named_assertion(
                    "Cancellable command has emitted its PTY ready marker",
                    assert_active_block_output_for_single_terminal_in_tab(
                        ExactLine::from(CANCEL_READY_MARKER),
                        1,
                    ),
                ),
        )
        .with_step(
            TestStep::new("Interrupt the shell command with Ctrl+C")
                .with_keystrokes(&["ctrl-c"])
                .add_assertion(assert_no_block_executing(1, 0))
                .add_assertion(assert_active_block_received_precmd(1, 0)),
        )
        .with_step(open_command_palette())
        .with_step(close_command_palette())
        .with_step(
            new_step_with_default_assertions("Open the local command history")
                .with_keystrokes(&["ctrl-r"])
                .add_named_assertion("Command search is open", assert_command_search_is_open()),
        )
        .with_step(
            new_step_with_default_assertions("Select the local history filter")
                .with_typed_characters(&["h"])
                .with_keystrokes(&["tab"])
                .add_named_assertion("History filter is active", assert_history_filter_is_active()),
        )
        .with_step(
            new_step_with_default_assertions("Find the command just executed in the real shell")
                .with_typed_characters(&[OUTPUT_MARKER])
                .add_assertion(assert_command_search_has_results()),
        )
        .with_step(
            new_step_with_default_assertions("Return to the terminal input")
                .with_keystrokes(&["escape"])
                .add_named_assertion("History search is closed", |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, _| {
                        async_assert!(!workspace.is_command_search_open())
                    })
                }),
        )
        .with_step(
            new_step_with_default_assertions("Save the local tab and command snapshot")
                .with_action(|app, _, _| {
                    app.dispatch_global_action("workspace:save_app", ());
                })
                .add_named_assertion("Lite keeps AI disabled", |app, _| {
                    AISettings::handle(app).read(app, |settings, ctx| {
                        async_assert!(
                            !settings.is_any_ai_enabled(ctx),
                            "Lite should keep AI disabled"
                        )
                    })
                })
                .add_named_assertion("Project tab keeps the directory containing spaces", |app, window_id| {
                    let expected = PathBuf::from(std::env::var_os(DIRECTORY_ENV).expect("test directory should be set"));
                    single_terminal_view_for_tab(app, window_id, 1).read(app, |view, _| {
                        let actual = view.pwd();
                        async_assert!(
                            actual.as_deref().is_some_and(|pwd| same_directory(pwd, &expected)),
                            "Expected project directory {expected:?}, got {actual:?}"
                        )
                    })
                })
                .add_named_assertion("Read-only history uses the same absolute database as the writer", |_, _| {
                    let expected = test_database_path();
                    let actual = database_file_path_for_current_scope();
                    async_assert!(
                        actual.is_absolute() && actual == expected,
                        "Expected the primary test database {expected:?}, got {actual:?}"
                    )
                })
                .add_named_assertion("SQLite contains the tab name, directory, and completed output", |_, _| {
                    let expected = PathBuf::from(std::env::var_os(DIRECTORY_ENV).expect("test directory should be set"));
                    let database_path = test_database_path();
                    let database_url = format!("file:{}?mode=ro", database_path.to_string_lossy().replace('\\', "/"));
                    let mut conn = SqliteConnection::establish(&database_url)
                        .unwrap_or_else(|error| panic!("should open snapshot {database_path:?}: {error}"));
                    conn.batch_execute("PRAGMA busy_timeout = 1000;")
                        .expect("should allow pending snapshot writes to finish");
                    let titles: Vec<_> = sql_query("SELECT custom_title FROM tabs ORDER BY id")
                        .load::<SavedTab>(&mut conn)
                        .expect("should read saved tab names")
                        .into_iter()
                        .map(|tab| tab.custom_title)
                        .collect();
                    let directories: Vec<_> = sql_query("SELECT cwd FROM terminal_panes")
                        .load::<SavedDirectory>(&mut conn)
                        .expect("should read saved terminal directories")
                        .into_iter()
                        .map(|pane| pane.cwd)
                        .collect();
                    let outputs = sql_query("SELECT pwd, stylized_output FROM blocks WHERE did_execute = 1")
                        .load::<SavedBlock>(&mut conn)
                        .expect("should read saved completed command blocks");
                    let groups = sql_query("SELECT name FROM tab_groups")
                        .load::<SavedGroup>(&mut conn)
                        .expect("should read saved project groups");
                    if titles.len() != 2
                        || titles[1].as_deref() != Some(TAB_TITLE)
                        || groups.len() != 1 || groups[0].name.is_some()
                        || !directories.iter().flatten().any(|pwd| same_directory(pwd, &expected))
                        || !outputs.iter().any(|block| {
                            block.pwd.as_deref().is_some_and(|pwd| same_directory(pwd, &expected))
                                && !block.stylized_output.is_empty()
                        })
                    {
                        return AssertionOutcome::failure(format!(
                            "Snapshot is not complete yet: titles={titles:?}, directories={directories:?}, completed_blocks={}",
                            outputs.len()
                        ));
                    }
                    let evidence = serde_json::json!({
                        "check": "live shell, tabs, cwd and SQLite snapshot",
                        "process_id": std::process::id(),
                        "database": database_path,
                        "titles": titles,
                        "directories": directories,
                        "completed_blocks": outputs.len(),
                        "automatic_directory_groups": groups.len(),
                        "visible_output": OUTPUT_MARKER,
                        "process_restart_tested": false,
                    });
                    let evidence_path = test_state_dir().join("lite-smoke-evidence.json");
                    std::fs::write(&evidence_path, serde_json::to_vec_pretty(&evidence).expect("evidence should serialize"))
                        .expect("should preserve smoke-test evidence");
                    println!("Lite smoke evidence: {}", evidence_path.display());
                    AssertionOutcome::Success
                }),
        );
    if std::env::var_os("WARPUI_USE_REAL_DISPLAY_IN_INTEGRATION_TESTS").is_some() {
        builder
            .with_step(
                TestStep::new(
                    "Capture the rendered Lite terminal after successful snapshot verification",
                )
                .with_take_screenshot("lite-terminal-smoke.png"),
            )
            .with_step(
                TestStep::new("Verify the rendered Lite terminal screenshot").add_named_assertion(
                    "The captured terminal PNG is readable",
                    |_, _| {
                        let Some(artifacts) = std::env::var_os(ARTIFACTS_DIR_ENV_VAR) else {
                            return AssertionOutcome::failure(
                                "Test artifacts directory is not set".to_owned(),
                            );
                        };
                        let test_artifacts =
                            PathBuf::from(artifacts).join("test_lite_terminal_smoke");
                        let latest_run =
                            std::fs::read_dir(&test_artifacts).ok().and_then(|entries| {
                                entries
                                    .filter_map(Result::ok)
                                    .filter(|entry| entry.path().is_dir())
                                    .max_by_key(|entry| entry.file_name())
                                    .map(|entry| entry.path())
                            });
                        let Some(run_directory) = latest_run else {
                            return AssertionOutcome::failure(format!(
                                "No timestamped screenshot run directory under {test_artifacts:?}"
                            ));
                        };
                        let screenshot = run_directory.join("lite-terminal-smoke.png");
                        let image = match std::fs::read(&screenshot) {
                            Ok(image) => image,
                            Err(error) => {
                                return AssertionOutcome::failure(format!(
                                    "Cannot read GPU-rendered screenshot {screenshot:?}: {error}"
                                ));
                            }
                        };
                        if !image.starts_with(b"\x89PNG\r\n\x1a\n") {
                            return AssertionOutcome::failure(format!(
                                "GPU-rendered screenshot {screenshot:?} is not a PNG"
                            ));
                        }
                        println!("Lite terminal screenshot: {}", screenshot.display());
                        AssertionOutcome::Success
                    },
                ),
            )
    } else {
        builder
    }
}

pub fn test_lite_restore_named_tabs() -> Builder {
    lite_builder()
        .with_setup(|utils| {
            let directory = prepare_directory(utils);
            let database_path = test_database_path();
            warp::integration_testing::create_file_from_assets(
                TEST_ONLY_ASSETS,
                "three_tabs.sqlite",
                &database_path,
            );
            let mut conn = SqliteConnection::establish(database_path.to_str().expect("database path should be UTF-8"))
                .expect("should open the test fixture database");
            sql_query("UPDATE tabs SET custom_title = ?")
                .bind::<Text, _>(TAB_TITLE)
                .execute(&mut conn)
                .expect("should give restored tabs a custom name");
            sql_query("UPDATE terminal_panes SET cwd = ?")
                .bind::<Text, _>(directory.to_string_lossy().into_owned())
                .execute(&mut conn)
                .expect("should give restored terminals a directory containing spaces");
        })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(wait_until_bootstrapped_single_pane_for_tab(1))
        .with_step(wait_until_bootstrapped_single_pane_for_tab(2))
        .with_step(
            new_step_with_default_assertions("Restore named terminal tabs from a saved fixture")
                .add_named_assertion("All three tabs restore their names and project directories", |app, window_id| {
                    let expected = PathBuf::from(std::env::var_os(DIRECTORY_ENV).expect("test directory should be set"));
                    let tab_count = workspace_view(app, window_id).read(app, |workspace, _| workspace.tab_count());
                    if tab_count != 3 {
                        return AssertionOutcome::failure(format!("Expected three restored tabs, got {tab_count}"));
                    }
                    for tab_index in 0..3 {
                        let title = pane_group_view(app, window_id, tab_index).read(app, |pane_group, ctx| pane_group.display_title(ctx));
                        let directory = single_terminal_view_for_tab(app, window_id, tab_index).read(app, |view, _| view.pwd());
                        if title != TAB_TITLE || !directory.as_deref().is_some_and(|pwd| same_directory(pwd, &expected)) {
                            return AssertionOutcome::failure(format!(
                                "Restored tab {tab_index}: expected title {TAB_TITLE:?} and directory {expected:?}, got {title:?} and {directory:?}"
                            ));
                        }
                    }
                    let evidence = serde_json::json!({
                        "check": "restore named tabs and cwd from a SQLite fixture",
                        "fixture": "three_tabs.sqlite",
                        "tab_count": tab_count,
                        "title": TAB_TITLE,
                        "directory": expected,
                        "process_restart_tested": false,
                    });
                    let evidence_path = test_state_dir().join("lite-restore-evidence.json");
                    std::fs::write(&evidence_path, serde_json::to_vec_pretty(&evidence).expect("evidence should serialize"))
                        .expect("should preserve fixture restoration evidence");
                    println!("Lite restoration evidence: {}", evidence_path.display());
                    AssertionOutcome::Success
                }),
        )
}

pub fn test_lite_restore_saved_tabs() -> Builder {
    let builder = lite_builder()
        .with_setup(|utils| {
            let source = PathBuf::from(
                std::env::var_os("WARP_LITE_RESTORE_DATABASE")
                    .expect("set WARP_LITE_RESTORE_DATABASE to the completed smoke-test backup"),
            )
            .canonicalize()
            .expect("the completed smoke-test backup should exist");
            let base_dirs = directories::BaseDirs::new().expect("Windows application data should exist");
            let in_test_namespace = source.components().any(|component| {
                component
                    .as_os_str()
                    .to_string_lossy()
                    .starts_with("WarpIntegrationLiteSmoke-")
            }) && [base_dirs.data_local_dir(), base_dirs.data_dir()]
                .iter()
                .filter_map(|path| path.canonicalize().ok())
                .any(|root| source.starts_with(root));
            let artifact_root = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/lite-test-artifacts")
                .canonicalize()
                .ok();
            assert!(
                source.is_file()
                    && (in_test_namespace
                        || artifact_root.is_some_and(|root| source.starts_with(root))),
                "Restore input must be a file in a Lite test namespace or target/lite-test-artifacts: {source:?}"
            );
            let directory = PathBuf::from(
                std::env::var_os("WARP_LITE_RESTORE_DIRECTORY")
                    .expect("set WARP_LITE_RESTORE_DIRECTORY to the first process's project directory"),
            );
            assert!(directory.is_dir(), "The saved project directory should still exist: {directory:?}");
            prepare_directory(utils);
            utils.set_env(DIRECTORY_ENV, Some(directory));
            let database_path = test_database_path();
            assert!(!database_path.exists(), "The restore process should have a new test database");
            std::fs::create_dir_all(database_path.parent().expect("database should have a parent"))
                .expect("should create the new test namespace");
            std::fs::copy(&source, &database_path).expect("should copy the completed smoke-test backup");
            if std::env::var_os("WARP_LITE_EXPECT_RESTORED_GROUPS").is_some() {
                let mut conn = SqliteConnection::establish(database_path.to_str().expect("test database path is UTF-8"))
                    .expect("should open the isolated restored snapshot");
                conn.batch_execute("UPDATE windows SET vertical_tabs_panel_open = 0;")
                    .expect("should reproduce an old snapshot with the sidebar closed");
            }
            println!("Restoring completed smoke snapshot from {}", source.display());
        })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(wait_until_bootstrapped_single_pane_for_tab(1))
        .with_step(assert_logged_out())
        .with_step(
            new_step_with_default_assertions("Restore the saved directory group when present")
                .add_named_assertion("The directory group and left sidebar survive restart", |app, window_id| {
                    if std::env::var_os("WARP_LITE_EXPECT_RESTORED_GROUPS").is_none() {
                        return AssertionOutcome::Success;
                    }
                    workspace_view(app, window_id).read(app, |workspace, ctx| {
                        let snapshot = workspace.snapshot(window_id, false, ctx);
                        let Some(group_id) = snapshot.tabs[1].group_id else {
                            return AssertionOutcome::failure("The restored project tab lost its group".to_owned());
                        };
                        async_assert!(snapshot.vertical_tabs_panel_open
                            && snapshot.tab_groups.len() == 1
                            && workspace.tab_group_display_name(group_id, ctx) == "Lite 项目 with spaces",
                            "The restored directory group should be visible in the left sidebar")
                    })
                }),
        )
        .with_step(
            new_step_with_default_assertions("Restore the previous process's saved project terminal")
                .add_named_assertion("Read-only history uses the same absolute database as the writer", |_, _| {
                    let expected = test_database_path();
                    let actual = database_file_path_for_current_scope();
                    async_assert!(
                        actual.is_absolute() && actual == expected,
                        "Expected the primary test database {expected:?}, got {actual:?}"
                    )
                })
                .add_named_assertion("The two saved tabs and project name survive restart", |app, window_id| {
                    let tab_count = workspace_view(app, window_id).read(app, |workspace, _| workspace.tab_count());
                    let title = pane_group_view(app, window_id, 1).read(app, |pane_group, ctx| pane_group.display_title(ctx));
                    async_assert!(
                        tab_count == 2 && title == TAB_TITLE,
                        "Expected two saved tabs and title {TAB_TITLE:?}, got {tab_count} tabs and {title:?}"
                    )
                })
                .add_named_assertion("The saved project cwd and command output are restored", |app, window_id| {
                    let expected = PathBuf::from(std::env::var_os(DIRECTORY_ENV).expect("test directory should be set"));
                    let (directory, restored_output) = single_terminal_view_for_tab(app, window_id, 1).read(app, |view, _| {
                        let directory = view.pwd();
                        let model = view.model.lock();
                        let restored_output = model.block_list().blocks().iter().any(|block| {
                            block.is_restored()
                                && block.command_to_string().contains("Write-Output")
                                && block.output_to_string().contains(OUTPUT_MARKER)
                        });
                        (directory, restored_output)
                    });
                    if !directory.as_deref().is_some_and(|pwd| same_directory(pwd, &expected)) || !restored_output {
                        return AssertionOutcome::failure(format!(
                            "Expected restored cwd {expected:?} and output {OUTPUT_MARKER:?}, got cwd {directory:?}, matching_restored_output={restored_output}"
                        ));
                    }
                    let evidence = serde_json::json!({
                        "check": "new process restores the completed smoke-test snapshot",
                        "source_database": std::env::var_os("WARP_LITE_RESTORE_DATABASE").map(PathBuf::from),
                        "restored_database": test_database_path(),
                        "process_id": std::process::id(),
                        "tab_count": 2,
                        "title": TAB_TITLE,
                        "directory": directory,
                        "restored_output": OUTPUT_MARKER,
                    });
                    let evidence_path = test_state_dir().join("lite-restart-evidence.json");
                    std::fs::write(&evidence_path, serde_json::to_vec_pretty(&evidence).expect("evidence should serialize"))
                        .expect("should preserve restart restoration evidence");
                    println!("Lite restart evidence: {}", evidence_path.display());
                    AssertionOutcome::Success
                }),
        );
    if std::env::var_os("WARPUI_USE_REAL_DISPLAY_IN_INTEGRATION_TESTS").is_some() {
        builder.with_step(
            TestStep::new("Capture the project terminal restored in a new process")
                .with_take_screenshot("lite-terminal-restored.png"),
        )
    } else {
        builder
    }
}

pub fn test_lite_wsl_directory_search() -> Builder {
    lite_builder().with_real_display()
        .with_setup(|utils| {
            prepare_directory(utils);
            // WSL must use the generated test rcfiles, not the user's Linux shell configuration.
            utils.set_env("WSLENV", Some("HOME/p"));
        })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(new_step_with_default_assertions("Wait for the installed WSL distro")
            .add_named_assertion("WSL is available", |app, _| app.read(|ctx| {
                async_assert!(AvailableShells::as_ref(ctx).get_available_shells().any(|shell| shell.is_wsl()), "Expected an installed WSL distribution")
            })))
        .with_step(new_step_with_default_assertions("Open a WSL terminal")
            .with_action(|app, window_id, _| {
                let shell = app.read(|ctx| AvailableShells::as_ref(ctx).get_available_shells().find(|shell| shell.is_wsl()).cloned().unwrap());
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::AddTabWithShell { shell, source: AddTabWithShellSource::ShellSelectorMenu });
            }))
        .with_step(wait_until_bootstrapped_single_pane_for_tab(1))
        .with_step(execute_command_for_single_terminal_in_tab(1,
            r#"fixture=$(mktemp -d "$HOME/warp-lite-search.XXXXXX"); mkdir "$fixture/搜索测试 目录"; touch "$fixture/readme.txt"; cd "$fixture""#.to_owned(), ExpectedExitStatus::Success, ()))
        .with_step(new_step_with_default_assertions("The WSL directory menu resolves its own home")
            .add_named_assertion("The tilde path lists WSL files", |app, window_id| {
                async_assert!(searchable_directory_chip(app, window_id).is_some(), "WSL home directory listing must contain the fixture")
            }))
        .with_step(new_step_with_default_assertions("Open the WSL directory menu")
            .with_action(|app, window_id, _| {
                let chip = searchable_directory_chip(app, window_id).unwrap();
                app.dispatch_typed_action(window_id, &[chip.id()], &DisplayChipAction::ToggleMenu);
            }))
        .with_step(new_step_with_default_assertions("Search WSL folders in Chinese")
            .with_typed_characters(&["搜索测试"])
            .add_named_assertion("Exactly the Chinese WSL directory matches", |app, window_id| {
                searchable_directory_menu(app, window_id).unwrap().read(app, |menu, _| async_assert_eq!(menu.filtered_item_names(), vec!["搜索测试 目录".to_owned()]))
            }).with_take_screenshot("lite-wsl-directory-search.png"))
        .with_step(new_step_with_default_assertions("Navigate to the WSL search result")
            .with_keystrokes(&["enter"])
            .add_named_assertion("The WSL shell changes directory", |app, window_id| {
                single_terminal_view_for_tab(app, window_id, 1).read(app, |view, _| async_assert!(view.pwd().as_deref().is_some_and(|pwd| pwd.ends_with("/搜索测试 目录")), "WSL navigation should use the selected directory"))
            }))
        .with_step(execute_command_for_single_terminal_in_tab(1,
            r#"cd "$fixture"; rmdir '搜索测试 目录'; rm readme.txt; cd ..; rmdir "$fixture""#.to_owned(), ExpectedExitStatus::Success, ()))
}
