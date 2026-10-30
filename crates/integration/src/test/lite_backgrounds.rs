use std::path::PathBuf;

use settings::Setting as _;
use warp::appearance::{Appearance, AppearanceManager};
use warp::integration_testing::step::new_step_with_default_assertions;
use warp::integration_testing::terminal::util::ExpectedExitStatus;
use warp::integration_testing::terminal::{
    execute_command_for_single_terminal_in_tab, wait_until_bootstrapped_single_pane_for_tab,
};
use warp::integration_testing::view_getters::workspace_view;
use warp::settings::ThemeSettings;
use warp::settings_view::{SettingsSection, SettingsView};
use warp::themes::theme::ThemeKind;
use warp::workspace::WorkspaceAction;
use warpui_core::assets::asset_cache::{AssetCache, AssetState};
use warpui_core::image_cache::ImageType;
use warpui_core::integration::{AssertionOutcome, TestSetupUtils};
use warpui_core::{App, SingletonEntity as _, async_assert};

use super::{assert_test_namespace, lite_builder, prepare_directory, test_state_dir};
use crate::Builder;

const BACKGROUND_DIRECTORY: &str = "WARP_LITE_TEST_BACKGROUND_DIR";
const PNG_A: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 16, 0, 0, 0, 16, 8, 6,
    0, 0, 0, 31, 243, 255, 97, 0, 0, 0, 44, 73, 68, 65, 84, 120, 156, 99, 16, 89, 245, 236, 63, 50,
    94, 165, 113, 2, 5, 19, 146, 103, 24, 6, 6, 144, 170, 1, 93, 126, 56, 24, 48, 240, 177, 48,
    240, 6, 12, 124, 44, 12, 184, 1, 0, 28, 192, 158, 31, 24, 135, 149, 226, 0, 0, 0, 0, 73, 69,
    78, 68, 174, 66, 96, 130,
];
const PNG_B: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 16, 0, 0, 0, 16, 8, 6,
    0, 0, 0, 31, 243, 255, 97, 0, 0, 0, 51, 73, 68, 65, 84, 120, 156, 99, 248, 176, 69, 242, 63,
    50, 214, 56, 145, 135, 130, 191, 70, 5, 160, 96, 116, 245, 12, 195, 192, 0, 82, 53, 160, 171,
    31, 14, 6, 144, 170, 1, 93, 253, 112, 48, 96, 224, 83, 226, 128, 27, 0, 0, 100, 60, 149, 15,
    55, 52, 199, 88, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];

const OVERSIZED_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 1, 0, 0, 0, 0, 0, 1, 8, 6, 0,
    0, 0, 107, 114, 227, 216, 0, 0, 1, 28, 73, 68, 65, 84, 120, 218, 237, 195, 65, 13, 0, 0, 8, 4,
    32, 67, 216, 63, 128, 193, 174, 134, 126, 13, 1, 27, 213, 147, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 253, 15, 160, 92, 39, 142, 113, 203, 106, 205, 0, 0, 0, 0, 73, 69, 78,
    68, 174, 66, 96, 130,
];

fn fixture_path() -> PathBuf {
    PathBuf::from(std::env::var_os(BACKGROUND_DIRECTORY).expect("isolated background directory"))
        .join("background.png")
}

fn prepare_background(utils: &mut TestSetupUtils, contents: Option<&[u8]>) {
    prepare_directory(utils);
    let directory = test_state_dir().join("fixed-background-fixtures");
    assert_test_namespace(&directory);
    assert!(
        !directory.exists(),
        "A background test must have a fresh namespace"
    );
    std::fs::create_dir_all(&directory).expect("create isolated background fixtures");
    utils.set_env(BACKGROUND_DIRECTORY, Some(directory));
    if let Some(contents) = contents {
        std::fs::write(fixture_path(), contents).expect("write the isolated fixed-name fixture");
    }
}

fn assert_background_image(app: &App) -> AssertionOutcome {
    app.read(|ctx| {
        let Some(image) = Appearance::as_ref(ctx).theme().background_image() else {
            return AssertionOutcome::failure("The valid startup background is missing".to_owned());
        };
        match AssetCache::as_ref(ctx).load_asset::<ImageType>(image.source) {
            AssetState::Loaded { data } => {
                let ImageType::StaticBitmap { image } = data.as_ref() else {
                    return AssertionOutcome::failure(
                        "The startup background must be a static bitmap".to_owned(),
                    );
                };
                async_assert!(
                    image.width() == 16
                        && image.height() == 16
                        && image.rgba_bytes().get(..4) == Some([20, 170, 230, 255].as_slice()),
                    "Expected the original startup pixels, got size {:?}, pixel {:?}",
                    image.size(),
                    image.rgba_bytes().get(..4)
                )
            }
            AssetState::FailedToLoad(error) => AssertionOutcome::failure(format!(
                "The renderer failed to load the startup background: {error}"
            )),
            AssetState::Loading { .. } | AssetState::Evicted => {
                AssertionOutcome::failure("The startup background is still loading".to_owned())
            }
        }
    })
}

fn assert_solid_background(app: &App) -> AssertionOutcome {
    app.read(|ctx| {
        async_assert!(
            Appearance::as_ref(ctx).theme().background_image().is_none(),
            "An unavailable startup background must keep the terminal on its solid color theme"
        )
    })
}

fn open_appearance() -> warpui_core::integration::TestStep {
    new_step_with_default_assertions("Open the uncluttered appearance page")
        .with_action(|app, window_id, _| {
            let workspace = workspace_view(app, window_id);
            app.dispatch_typed_action(
                window_id,
                &[workspace.id()],
                &WorkspaceAction::ShowSettingsPage(SettingsSection::Appearance),
            );
        })
        .add_named_assertion(
            "The appearance page no longer offers live background controls",
            |app, window_id| {
                let settings = app
                    .views_of_type::<SettingsView>(window_id)
                    .and_then(|views| views.into_iter().next());
                let Some(settings) = settings else {
                    return AssertionOutcome::failure(
                        "Appearance settings are not open".to_owned(),
                    );
                };
                let controls = app.read(|ctx| {
                    [
                        "lite-background-dropdown",
                        "lite-background-refresh",
                        "lite-background-browse",
                        "lite-background-open-directory",
                    ]
                    .map(|id| ctx.element_position_by_id_at_last_frame(window_id, id))
                });
                settings.read(app, |settings, _| {
                    async_assert!(
                        settings.current_settings_section() == SettingsSection::Appearance
                            && controls.iter().all(Option::is_none),
                        "Live background controls must be absent, got {controls:?}"
                    )
                })
            },
        )
}

fn real_theme_clicks(mut builder: Builder) -> Builder {
    builder = builder
        .with_step(
            new_step_with_default_assertions(
                "Click the current theme card through real mouse events",
            )
            .with_click_on_saved_position("appearance-theme-current")
            .add_named_assertion(
                "The theme chooser opens from the appearance card",
                |app, window_id| {
                    workspace_view(app, window_id).read(app, |workspace, _| {
                        async_assert!(
                            workspace.is_theme_chooser_open(),
                            "The theme chooser did not open from the mouse click"
                        )
                    })
                },
            ),
        )
        .with_step(
            new_step_with_default_assertions(
                "The Lite theme chooser exposes no image-theme editor",
            )
            .add_named_assertion(
                "The custom theme creation button is absent",
                |app, window_id| {
                    let bounds = app.read(|ctx| {
                        ctx.element_position_by_id_at_last_frame(window_id, "create_theme_button")
                    });
                    async_assert!(
                        bounds.is_none(),
                        "Lite must not expose the removed image-theme creation workflow"
                    )
                },
            ),
        )
        .with_step(
            new_step_with_default_assertions("Type into the live theme search field")
                .with_input_string("Dracula", None)
                .add_named_assertion(
                    "The Dracula theme is rendered and clickable",
                    |app, window_id| {
                        let bounds = app.read(|ctx| {
                            ctx.element_position_by_id_at_last_frame(
                                window_id,
                                "theme-chooser-item:Dracula",
                            )
                        });
                        async_assert!(
                            bounds
                                .is_some_and(|bounds| bounds.width() > 0. && bounds.height() > 0.),
                            "The filtered theme item is missing: {bounds:?}"
                        )
                    },
                ),
        )
        .with_step(
            new_step_with_default_assertions(
                "Choose an internal color theme through the real theme list",
            )
            .with_click_on_saved_position("theme-chooser-item:Dracula")
            .add_named_assertion(
                "The mouse click persists Dracula without a circular view update",
                |app, _| {
                    app.read(|ctx| {
                        async_assert!(
                            *ThemeSettings::as_ref(ctx).theme_kind.value() == ThemeKind::Dracula,
                            "The actual theme selection was not saved"
                        )
                    })
                },
            )
            .add_named_assertion(
                "The fixed startup image remains valid after a real theme click",
                |app, _| assert_background_image(app),
            ),
        )
        .with_step(
            new_step_with_default_assertions(
                "Close the theme chooser through its actual close button",
            )
            .with_click_on_saved_position("theme-chooser-close")
            .add_named_assertion(
                "Closing the chooser returns to the appearance page",
                |app, window_id| {
                    let chooser_closed = workspace_view(app, window_id)
                        .read(app, |workspace, _| !workspace.is_theme_chooser_open());
                    let settings = app
                        .views_of_type::<SettingsView>(window_id)
                        .and_then(|views| views.into_iter().next());
                    let Some(settings) = settings else {
                        return AssertionOutcome::failure(
                            "The appearance page disappeared after closing the chooser".to_owned(),
                        );
                    };
                    settings.read(app, |settings, _| {
                        async_assert!(
                            chooser_closed
                                && settings.current_settings_section()
                                    == SettingsSection::Appearance,
                            "Theme chooser must close while keeping appearance settings available"
                        )
                    })
                },
            )
            .with_take_screenshot("lite-theme-real-click.png"),
        );
    for query in ["background", "壁纸"] {
        builder = builder.with_step(
            new_step_with_default_assertions(
                "Removed background settings no longer appear in search",
            )
            .with_action(move |app, window_id, _| {
                let settings = app
                    .views_of_type::<SettingsView>(window_id)
                    .and_then(|views| views.into_iter().next())
                    .expect("settings page is available");
                settings.update(app, |settings, ctx| settings.set_search_query(query, ctx));
            })
            .add_named_assertion(
                "The appearance page advertises no removed background controls",
                |app, window_id| {
                    let settings = app
                        .views_of_type::<SettingsView>(window_id)
                        .and_then(|views| views.into_iter().next())
                        .expect("settings view stays alive during search");
                    settings.read(app, |settings, ctx| {
                        async_assert!(
                            !settings
                                .visible_sections_for_integration_test(ctx)
                                .contains(&SettingsSection::Appearance),
                            "Removed background controls must not match settings search"
                        )
                    })
                },
            ),
        );
    }
    builder
        .with_step(
            new_step_with_default_assertions(
                "Return to the uncluttered appearance page after searching",
            )
            .with_action(|app, window_id, _| {
                let settings = app
                    .views_of_type::<SettingsView>(window_id)
                    .and_then(|views| views.into_iter().next())
                    .expect("settings page is available");
                settings.update(app, |settings, ctx| settings.set_search_query("", ctx));
            }),
        )
        .with_step(open_appearance().with_take_screenshot("lite-theme-real-click-final.png"))
}

fn record_evidence(case: &'static str, has_image: bool) -> warpui_core::integration::TestStep {
    new_step_with_default_assertions("Record isolated background startup evidence")
        .with_action(move |app, _, _| {
            let actual = app.read(|ctx| Appearance::as_ref(ctx).theme().background_image().is_some());
            assert_eq!(actual, has_image);
            let report = serde_json::json!({"case": case, "process_id": std::process::id(), "fixture": fixture_path(), "startup_image": has_image, "live_file_changes_ignored": true, "background_controls_absent": true, "real_theme_mouse_click_tested": case == "valid-fixed-png", "removed_background_search_tested": case == "valid-fixed-png"});
            let path = test_state_dir().join("background-verification.json");
            std::fs::write(&path, serde_json::to_vec_pretty(&report).expect("serialize startup evidence"))
                .expect("write startup evidence in the isolated namespace");
            println!("Lite background startup evidence: {}", path.display());
        })
}

pub fn test_lite_fixed_background() -> Builder {
    let mut builder = lite_builder()
        .with_real_display()
        .with_setup(|utils| prepare_background(utils, Some(PNG_A)))
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(
            new_step_with_default_assertions("Load the fixed background during startup")
                .add_named_assertion("The renderer displays the fixed PNG", |app, _| {
                    assert_background_image(app)
                })
                .with_take_screenshot("lite-fixed-background-startup.png"),
        )
        .with_step(
            new_step_with_default_assertions("Replace the file without a live reload").with_action(
                |_, _, _| {
                    std::fs::write(fixture_path(), PNG_B)
                        .expect("replace only the isolated startup fixture")
                },
            ),
        );
    for theme in [
        ThemeKind::Light,
        ThemeKind::Dracula,
        ThemeKind::FancyDracula,
        ThemeKind::Dark,
    ] {
        builder = builder.with_step(
            new_step_with_default_assertions(
                "Switch color themes without reloading background textures",
            )
            .with_action(move |app, _, _| {
                AppearanceManager::handle(app).update(app, |manager, ctx| {
                    manager.set_transient_theme(theme.clone(), ctx)
                })
            })
            .add_named_assertion(
                "The original startup pixels remain stable across theme changes",
                |app, _| assert_background_image(app),
            ),
        );
    }
    let builder = builder
        .with_step(
            new_step_with_default_assertions(
                "Remove the file without invalidating the startup snapshot",
            )
            .with_action(|app, _, _| {
                std::fs::remove_file(fixture_path())
                    .expect("remove only the isolated startup fixture");
                AppearanceManager::handle(app)
                    .update(app, |manager, ctx| manager.clear_transient_theme(ctx));
            })
            .add_named_assertion(
                "A removed source cannot trigger a failed texture reload",
                |app, _| assert_background_image(app),
            ),
        )
        .with_step(open_appearance().with_take_screenshot("lite-fixed-background-appearance.png"));
    real_theme_clicks(builder).with_step(record_evidence("valid-fixed-png", true))
}

fn unavailable_background(contents: Option<&'static [u8]>, case: &'static str) -> Builder {
    lite_builder()
        .with_real_display()
        .with_setup(move |utils| prepare_background(utils, contents))
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(
            new_step_with_default_assertions("Start safely with an unavailable fixed background")
                .add_named_assertion("The terminal uses a solid theme after startup", |app, _| {
                    assert_solid_background(app)
                }),
        )
        .with_step(execute_command_for_single_terminal_in_tab(
            0,
            "Write-Output 'background-fallback-ok'".to_owned(),
            ExpectedExitStatus::Success,
            "background-fallback-ok".to_owned(),
        ))
        .with_step(
            new_step_with_default_assertions(
                "Adding a file does not activate live background loading",
            )
            .with_action(|app, _, _| {
                std::fs::write(fixture_path(), PNG_A)
                    .expect("write only the isolated replacement fixture");
                AppearanceManager::handle(app).update(app, |manager, ctx| {
                    manager.set_transient_theme(ThemeKind::Dracula, ctx)
                });
            })
            .add_named_assertion(
                "The running terminal remains on its startup solid-background policy",
                |app, _| assert_solid_background(app),
            ),
        )
        .with_step(open_appearance().with_take_screenshot("lite-solid-background-appearance.png"))
        .with_step(record_evidence(case, false))
}

pub fn test_lite_missing_background() -> Builder {
    unavailable_background(None, "missing-fixed-png")
}

pub fn test_lite_corrupt_background() -> Builder {
    unavailable_background(Some(b"This file is not a PNG image"), "corrupt-fixed-png")
}

pub fn test_lite_oversized_background() -> Builder {
    unavailable_background(Some(OVERSIZED_PNG), "oversized-fixed-png-65536x1")
}
