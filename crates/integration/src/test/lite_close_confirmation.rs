use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use settings::Setting as _;
use warp::integration_testing::command_palette::{
    assert_command_palette_has_results, assert_command_palette_is_closed,
};
use warp::integration_testing::step::new_step_with_default_assertions;
use warp::integration_testing::terminal::{
    assert_long_running_block_executing_for_single_terminal_in_tab, assert_no_block_executing,
    wait_until_bootstrapped_single_pane_for_tab,
};
use warp::integration_testing::view_getters::{
    command_palette_view, pane_group_view, single_terminal_view_for_tab, workspace_view,
};
use warp::integration_testing::workspace::{assert_focused_tab_index, assert_tab_count};
use warp::settings_view::SettingsSection;
use warp::terminal::general_settings::GeneralSettings;
use warp::workspace::WorkspaceAction;
use warpui_core::integration::{AssertionOutcome, TestStep};
use warpui_core::platform::TerminationMode;
use warpui_core::{App, EntityId, SingletonEntity as _, WindowId, async_assert};

use super::{lite_builder, prepare_directory, test_state_dir};
use crate::Builder;

type OriginalTerminal = Arc<Mutex<Option<(EntityId, EntityId)>>>;

fn original_terminal_is_intact(
    app: &App,
    window_id: WindowId,
    original: &OriginalTerminal,
) -> AssertionOutcome {
    let expected = *original.lock().expect("original terminal identity");
    let actual = (
        pane_group_view(app, window_id, 0).id(),
        single_terminal_view_for_tab(app, window_id, 0).id(),
    );
    let tabs = workspace_view(app, window_id).read(app, |workspace, _| workspace.tab_count());
    async_assert!(
        app.window_ids().contains(&window_id) && tabs == 1 && expected == Some(actual),
        "Cancellation must retain the original window, tab and pane: expected {expected:?}, got {actual:?}, tabs={tabs}"
    )
}

fn modal_is_open(app: &mut App, window_id: WindowId) -> AssertionOutcome {
    workspace_view(app, window_id).read(app, |workspace, ctx| {
        let bounds = ctx.element_position_by_id_at_last_frame(window_id, "native-modal-dialog");
        async_assert!(
            workspace.is_native_quit_modal_open(ctx)
                && bounds.is_some_and(|bounds| bounds.width() > 0. && bounds.height() > 0.),
            "An explicit close request must render a visible confirmation dialog: {bounds:?}"
        )
    })
}

fn modal_is_closed(app: &mut App, window_id: WindowId) -> AssertionOutcome {
    workspace_view(app, window_id).read(app, |workspace, ctx| {
        async_assert!(
            !workspace.is_native_quit_modal_open(ctx),
            "The cancelled close confirmation must be dismissed"
        )
    })
}

fn request_close(action: WorkspaceAction, name: &'static str) -> TestStep {
    new_step_with_default_assertions(name)
        .with_action(move |app, window_id, _| {
            let workspace = workspace_view(app, window_id);
            app.dispatch_typed_action(window_id, &[workspace.id()], &action);
        })
        .add_named_assertion("A close confirmation is visible", modal_is_open)
}

fn process_list_is_visible_and_focused(app: &mut App, window_id: WindowId) -> AssertionOutcome {
    let palette = command_palette_view(app, window_id);
    let focused = app.update(|ctx| palette.is_self_or_child_focused(ctx));
    workspace_view(app, window_id).read(app, |workspace, ctx| {
        let bounds = ctx.element_position_by_id_at_last_frame(window_id, "command_palette:list");
        async_assert!(
            workspace.is_palette_open() && focused
                && bounds.is_some_and(|bounds| bounds.width() > 0. && bounds.height() > 0.),
            "Running processes must remain visible and focused after modal dismissal: focused={focused}, bounds={bounds:?}"
        )
    })
}

fn show_running_processes(name: &'static str, screenshot: &'static str) -> TestStep {
    TestStep::new(name)
        .with_click_on_saved_position("native-modal-button:1")
        .add_named_assertion("The quit confirmation is dismissed", modal_is_closed)
        .add_named_assertion(
            "The process list is visible and retains keyboard focus",
            process_list_is_visible_and_focused,
        )
        .add_assertion(assert_command_palette_has_results())
        .add_named_assertion(
            "Viewing processes leaves the command running",
            assert_long_running_block_executing_for_single_terminal_in_tab(true, 0),
        )
        .with_take_screenshot(screenshot)
}

pub fn test_lite_close_confirmation() -> Builder {
    let original: OriginalTerminal = Default::default();
    let capture = original.clone();
    let last_tab_open = original.clone();
    let last_tab_cancelled = original.clone();
    let window_open = original.clone();
    let window_cancelled = original.clone();
    let quit_cancelled = original.clone();
    let keyboard_quit_cancelled = original.clone();
    let final_original = original.clone();
    let confirmed_original = original.clone();
    lite_builder().with_real_display()
        .with_user_defaults(HashMap::from([("ShowWarningBeforeQuitting".to_owned(), "false".to_owned())]))
        .with_setup(|utils| { prepare_directory(utils); })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(new_step_with_default_assertions("Keep close confirmations mandatory even with the legacy warning disabled")
            .with_action(move |app, window_id, _| {
                *capture.lock().expect("capture original identities") = Some((pane_group_view(app, window_id, 0).id(), single_terminal_view_for_tab(app, window_id, 0).id()));
                GeneralSettings::handle(app).update(app, |settings, ctx| settings.show_warning_before_quitting.set_value(false, ctx).expect("reproduce the old disabled-warning setting"));
            })
            .add_named_assertion("The legacy warning setting is disabled for the regression", |app, _| app.read(|ctx| async_assert!(!*GeneralSettings::as_ref(ctx).show_warning_before_quitting.value()))))
        .with_step(request_close(WorkspaceAction::CloseActiveTab, "Request closing the last idle tab")
            .add_named_assertion("The last tab and pane have not been removed before confirmation", move |app, window_id| original_terminal_is_intact(app, window_id, &last_tab_open))
            .with_take_screenshot("lite-confirm-last-tab.png"))
        .with_step(new_step_with_default_assertions("Cancel closing the final tab with a real Escape key")
            .with_keystrokes(&["escape"])
            .add_named_assertion("Escape dismisses the confirmation", modal_is_closed)
            .add_named_assertion("Escape retains the original terminal", move |app, window_id| original_terminal_is_intact(app, window_id, &last_tab_cancelled)))
        .with_step(new_step_with_default_assertions("Use the cancellable platform window-close route used by X and Alt-F4")
            .with_action(|app, window_id, _| app.update(|ctx| ctx.windows().close_window(window_id, TerminationMode::Cancellable)))
            .add_named_assertion("Window close also asks for confirmation", modal_is_open)
            .add_named_assertion("The window close has not destroyed any terminal", move |app, window_id| original_terminal_is_intact(app, window_id, &window_open)))
        .with_step(new_step_with_default_assertions("Repeated window close cannot bypass an open confirmation")
            .with_action(|app, window_id, _| app.update(|ctx| ctx.windows().close_window(window_id, TerminationMode::Cancellable)))
            .add_named_assertion("Repeated X leaves a single usable confirmation", modal_is_open)
            .with_take_screenshot("lite-confirm-window-close.png"))
        .with_step(new_step_with_default_assertions("Click the actual Chinese cancel button")
            .with_click_on_saved_position("native-modal-button:1")
            .add_named_assertion("The cancel button dismisses the close request", modal_is_closed)
            .add_named_assertion("The cancel button keeps the terminal identities", move |app, window_id| original_terminal_is_intact(app, window_id, &window_cancelled)))
        .with_step(request_close(WorkspaceAction::TerminateApp, "The normal quit command also requires confirmation")
            .with_take_screenshot("lite-confirm-quit.png"))
        .with_step(new_step_with_default_assertions("Cancel the quit command without losing the original terminal")
            .with_keystrokes(&["escape"])
            .add_named_assertion("Quit cancellation dismisses the dialog", modal_is_closed)
            .add_named_assertion("Quit cancellation retains the original terminal", move |app, window_id| original_terminal_is_intact(app, window_id, &quit_cancelled)))
        .with_step(new_step_with_default_assertions("Use the actual Ctrl-Shift-Q quit keybinding")
            .with_keystrokes(&["ctrl-shift-Q"])
            .add_named_assertion("The quit shortcut opens the same mandatory confirmation", modal_is_open))
        .with_step(new_step_with_default_assertions("Cancel the keyboard quit request with a real Escape key")
            .with_keystrokes(&["escape"])
            .add_named_assertion("The keyboard quit dialog is dismissed", modal_is_closed)
            .add_named_assertion("The keyboard quit cancellation preserves the terminal", move |app, window_id| original_terminal_is_intact(app, window_id, &keyboard_quit_cancelled)))
        .with_step(TestStep::new("Start a process before testing the three-button quit dialog")
            .with_typed_characters(&["Start-Sleep -Seconds 300"])
            .with_keystrokes(&["enter"])
            .add_named_assertion("A foreground process is running", assert_long_running_block_executing_for_single_terminal_in_tab(true, 0)))
        .with_step(TestStep::new("Quit with a running process")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::TerminateApp);
            })
            .add_named_assertion("Running processes require a visible confirmation", modal_is_open)
            .with_take_screenshot("lite-confirm-running-process.png"))
        .with_step(show_running_processes("View running processes from a terminal tab", "lite-processes-from-terminal.png"))
        .with_step(TestStep::new("Request quit again with the process list open")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::TerminateApp);
            })
            .add_named_assertion("Quit confirmation can cover the process list", modal_is_open))
        .with_step(TestStep::new("Cancel quitting and return focus to the existing process list")
            .with_keystrokes(&["escape"])
            .add_named_assertion("The modal closes", modal_is_closed)
            .add_named_assertion("The process list regains focus", process_list_is_visible_and_focused))
        .with_step(TestStep::new("Dismiss the process list with Escape")
            .with_keystrokes(&["escape"])
            .add_assertion(assert_command_palette_is_closed())
            .add_named_assertion("The terminal stays active", assert_focused_tab_index(0)))
        .with_step(TestStep::new("Open settings while the terminal command is running")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::ShowSettingsPage(SettingsSection::Appearance));
            })
            .add_named_assertion("Settings is the active tab", assert_focused_tab_index(1)))
        .with_step(TestStep::new("Request platform window close from settings")
            .with_action(|app, window_id, _| app.update(|ctx| ctx.windows().close_window(window_id, TerminationMode::Cancellable)))
            .add_named_assertion("The same quit confirmation is visible", modal_is_open))
        .with_step(show_running_processes("View running processes from settings", "lite-processes-from-settings.png"))
        .with_step(TestStep::new("Navigate to the running terminal with Enter")
            .with_keystrokes(&["enter"])
            .add_assertion(assert_command_palette_is_closed())
            .add_named_assertion("The selected process activates its terminal", assert_focused_tab_index(0))
            .add_named_assertion("Navigation does not stop the command", assert_long_running_block_executing_for_single_terminal_in_tab(true, 0)))
        .with_step(TestStep::new("Close the temporary settings tab")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                workspace.update(app, |workspace, ctx| workspace.activate_tab(1, ctx));
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::CloseActiveTab);
            })
            .add_named_assertion("Only the original terminal remains", assert_tab_count(1))
            .add_named_assertion("The original terminal is active", assert_focused_tab_index(0)))
        .with_step(TestStep::new("Quit again after inspecting processes")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::TerminateApp);
            })
            .add_named_assertion("Quit confirmation remains usable", modal_is_open))
        .with_step(TestStep::new("Cancel quitting while the process remains running")
            .with_keystrokes(&["escape"])
            .add_named_assertion("The process survives cancellation", assert_long_running_block_executing_for_single_terminal_in_tab(true, 0)))
        .with_step(new_step_with_default_assertions("Stop the test process")
            .with_keystrokes(&["ctrl-c"])
            .add_named_assertion("The terminal is idle again", assert_no_block_executing(0, 0)))
        .with_step(new_step_with_default_assertions("Create another terminal to verify a confirmed close affects one tab")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::AddTerminalTab { hide_homepage: true });
            })
            .add_named_assertion("The second tab is ready to be closed", assert_tab_count(2))
            .add_named_assertion("The second tab is focused", assert_focused_tab_index(1)))
        .with_step(wait_until_bootstrapped_single_pane_for_tab(1))
        .with_step(new_step_with_default_assertions("Close a regular tab immediately without confirmation")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::CloseActiveTab);
            })
            .add_named_assertion("Exactly one tab closes immediately", assert_tab_count(1))
            .add_named_assertion("The original tab regains focus", assert_focused_tab_index(0))
            .add_named_assertion("Closing a tab retains the original terminal", move |app, window_id| original_terminal_is_intact(app, window_id, &confirmed_original))
            .add_named_assertion("Ordinary tab closure never opens a confirmation", modal_is_closed))
        .with_step(new_step_with_default_assertions("A subsequent Enter cannot replay the stale close callback")
            .with_keystrokes(&["enter"])
            .add_named_assertion("The original terminal remains after the confirmed close", move |app, window_id| original_terminal_is_intact(app, window_id, &final_original))
            .add_named_assertion("No stale close confirmation remains", modal_is_closed))
        .with_step(new_step_with_default_assertions("Record mandatory close-confirmation evidence")
            .with_action(|_, _, _| {
                let path = test_state_dir().join("close-confirmation-verification.json");
                let report = serde_json::json!({"process_id": std::process::id(), "legacy_warning_disabled": true, "idle_last_tab_confirmation": true, "last_tab_preserved_before_confirmation": true, "real_escape_cancel": true, "cancellable_platform_window_close": true, "repeated_window_close": true, "real_cancel_button": true, "quit_command_confirmation": true, "real_ctrl_shift_q_confirmation": true, "ordinary_tab_closes_without_confirmation": true, "original_terminal_identity_preserved": true});
                std::fs::write(&path, serde_json::to_vec_pretty(&report).expect("serialize close-confirmation evidence")).expect("write isolated close-confirmation evidence");
                println!("Lite close confirmation evidence: {}", path.display());
            }))
}
