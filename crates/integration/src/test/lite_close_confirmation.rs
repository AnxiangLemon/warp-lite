use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use settings::Setting as _;
use warp::integration_testing::step::new_step_with_default_assertions;
use warp::integration_testing::terminal::wait_until_bootstrapped_single_pane_for_tab;
use warp::integration_testing::view_getters::{
    pane_group_view, single_terminal_view_for_tab, workspace_view,
};
use warp::integration_testing::workspace::{assert_focused_tab_index, assert_tab_count};
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
    let reordered_original = original.clone();
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
        .with_step(new_step_with_default_assertions("Create another terminal to verify a confirmed close affects one tab")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::AddTerminalTab { hide_homepage: true });
            })
            .add_named_assertion("The second tab is ready to be closed", assert_tab_count(2))
            .add_named_assertion("The second tab is focused", assert_focused_tab_index(1)))
        .with_step(wait_until_bootstrapped_single_pane_for_tab(1))
        .with_step(request_close(WorkspaceAction::CloseActiveTab, "Ask before closing the second idle tab")
            .add_named_assertion("The close request leaves both tabs intact", assert_tab_count(2)))
        .with_step(new_step_with_default_assertions("Reorder the selected tab while its close confirmation is open")
            .with_action(|app, window_id, _| {
                let workspace = workspace_view(app, window_id);
                app.dispatch_typed_action(window_id, &[workspace.id()], &WorkspaceAction::MoveActiveTabLeft);
            })
            .add_named_assertion("The close target moved to the first tab", assert_focused_tab_index(0))
            .add_named_assertion("Reordering does not dismiss the pending close confirmation", modal_is_open)
            .add_named_assertion("The original terminal moved to the second index while retaining its identity", move |app, window_id| {
                let expected = *reordered_original.lock().expect("original identities before reordering");
                let actual = (pane_group_view(app, window_id, 1).id(), single_terminal_view_for_tab(app, window_id, 1).id());
                async_assert!(expected == Some(actual), "The original terminal must move to index 1: expected {expected:?}, got {actual:?}")
            })
            .add_named_assertion("Both tabs remain before the reordered target is confirmed", assert_tab_count(2)))
        .with_step(new_step_with_default_assertions("Confirm one tab close with a real button click")
            .with_click_on_saved_position("native-modal-button:0")
            .add_named_assertion("Exactly one tab closes after confirmation", assert_tab_count(1))
            .add_named_assertion("The original tab regains focus", assert_focused_tab_index(0))
            .add_named_assertion("Confirmation closes the intended target after reordering, retaining the original terminal", move |app, window_id| original_terminal_is_intact(app, window_id, &confirmed_original))
            .add_named_assertion("The confirmation closes after one action", modal_is_closed))
        .with_step(new_step_with_default_assertions("A subsequent Enter cannot replay the stale close callback")
            .with_keystrokes(&["enter"])
            .add_named_assertion("The original terminal remains after the confirmed close", move |app, window_id| original_terminal_is_intact(app, window_id, &final_original))
            .add_named_assertion("No stale close confirmation remains", modal_is_closed))
        .with_step(new_step_with_default_assertions("Record mandatory close-confirmation evidence")
            .with_action(|_, _, _| {
                let path = test_state_dir().join("close-confirmation-verification.json");
                let report = serde_json::json!({"process_id": std::process::id(), "legacy_warning_disabled": true, "idle_last_tab_confirmation": true, "last_tab_preserved_before_confirmation": true, "real_escape_cancel": true, "cancellable_platform_window_close": true, "repeated_window_close": true, "real_cancel_button": true, "quit_command_confirmation": true, "real_ctrl_shift_q_confirmation": true, "real_confirm_button_closes_one_tab": true, "tab_reorder_during_confirmation": true, "original_terminal_identity_preserved": true});
                std::fs::write(&path, serde_json::to_vec_pretty(&report).expect("serialize close-confirmation evidence")).expect("write isolated close-confirmation evidence");
                println!("Lite close confirmation evidence: {}", path.display());
            }))
}
