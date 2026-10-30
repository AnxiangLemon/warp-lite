use pathfinder_geometry::vector::vec2f;
use warp_core::ui::theme::Fill;
use warp_errors::report_error;
use warpui::elements::{
    Align, ChildAnchor, Container, MouseStateHandle, OffsetPositioning, ParentAnchor,
    ParentOffsetBounds, Stack,
};
use warpui::fonts::Weight;
use warpui::platform::Cursor;
use warpui::ui_components::button::ButtonVariant;
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::ui_components::text::Span;
use warpui::{
    AppContext, Element, Entity, EntityId, SingletonEntity, TypedActionView, View, ViewContext,
};

use crate::appearance::Appearance;
use crate::lite_i18n::choose;
use crate::pane_group::PaneId;
use crate::ui_components::dialog::{Dialog, dialog_styles};
use crate::workspace::TabMovement;

pub(super) fn init(app: &mut AppContext) {
    use warpui::keymap::FixedBinding;
    use warpui::keymap::macros::*;

    app.register_fixed_bindings([
        FixedBinding::new(
            "escape",
            CloseSessionConfirmationAction::Cancel,
            id!("CloseSessionConfirmation"),
        ),
        FixedBinding::new(
            "enter",
            CloseSessionConfirmationAction::CloseSession {
                dont_show_again: false,
            },
            id!("CloseSessionConfirmation"),
        ),
    ]);
}

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone)]
/// Describes the action which opened the close session confirmation dialog
pub enum OpenDialogSource {
    /// Close a specific pane
    ClosePane {
        pane_group_id: EntityId,
        pane_id: PaneId,
    },
    /// Close a specific tab
    CloseTab { tab_index: usize },
    /// Close all tabs other than the tab_index
    CloseOtherTabs { tab_index: usize },
    /// Close all tabs to the right/left of tab_index
    CloseTabsDirection {
        tab_index: usize,
        direction: TabMovement,
    },
}

pub struct CloseSessionConfirmationDialog {
    cancel_mouse_state: MouseStateHandle,
    confirm_mouse_state: MouseStateHandle,
    dont_show_again_mouse_state: MouseStateHandle,
    dont_show_again: bool,
    // Source will be None if dialog was never opened, since there is no reasonable default
    open_confirmation_source: Option<OpenDialogSource>,
}

#[allow(dead_code)]
impl CloseSessionConfirmationDialog {
    pub fn new() -> Self {
        Self {
            cancel_mouse_state: Default::default(),
            confirm_mouse_state: Default::default(),
            dont_show_again_mouse_state: Default::default(),
            open_confirmation_source: None,
            dont_show_again: false,
        }
    }
    pub fn set_open_confirmation_source(&mut self, source: OpenDialogSource) {
        self.open_confirmation_source = Some(source);
    }

    pub fn get_open_confirmation_source(&self) -> Option<OpenDialogSource> {
        self.open_confirmation_source
    }
}

impl Entity for CloseSessionConfirmationDialog {
    type Event = CloseSessionConfirmationEvent;
}

impl View for CloseSessionConfirmationDialog {
    fn ui_name() -> &'static str {
        "CloseSessionConfirmation"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let button_style = UiComponentStyles {
            font_size: Some(14.),
            font_weight: Some(Weight::Bold),
            width: Some(202.),
            height: Some(40.),
            ..Default::default()
        };

        let dont_show_again_checkbox = appearance
            .ui_builder()
            .checkbox(self.dont_show_again_mouse_state.clone(), Some(14.))
            .with_label(Span::new(
                choose("Don't show again.", "不再提示。"),
                Default::default(),
            ))
            .check(self.dont_show_again)
            .build()
            .with_cursor(Cursor::PointingHand)
            .on_click(|ctx, _, _| {
                ctx.dispatch_typed_action(CloseSessionConfirmationAction::ToggleDontShowAgain)
            })
            .finish();

        let dont_show_again_value = !cfg!(feature = "warp_lite") && self.dont_show_again;
        let close_session_button = appearance
            .ui_builder()
            .button(ButtonVariant::Accent, self.confirm_mouse_state.clone())
            .with_centered_text_label(choose("Close session", "确认关闭").into())
            .with_style(button_style)
            .build()
            .with_cursor(Cursor::PointingHand)
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(CloseSessionConfirmationAction::CloseSession {
                    dont_show_again: dont_show_again_value,
                })
            })
            .finish();

        let cancel_button = appearance
            .ui_builder()
            .button(ButtonVariant::Basic, self.cancel_mouse_state.clone())
            .with_centered_text_label(choose("Cancel", "取消").into())
            .with_style(button_style)
            .build()
            .with_cursor(Cursor::PointingHand)
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(CloseSessionConfirmationAction::Cancel)
            })
            .finish();

        #[cfg(feature = "integration_tests")]
        let close_session_button =
            warpui::elements::SavePosition::new(close_session_button, "close-session-confirm")
                .finish();
        #[cfg(feature = "integration_tests")]
        let cancel_button =
            warpui::elements::SavePosition::new(cancel_button, "close-session-cancel").finish();
        let mut dialog = Dialog::new(
                choose("Close session?", "关闭分屏？").into(),
                Some(choose(
                    "You are about to close a session that is currently being shared. Closing it will end sharing for everyone.",
                    "关闭后会结束此分屏中的终端会话。",
                ).into()),
                UiComponentStyles {
                    width: Some(460.),
                    padding: Some(Coords::uniform(24.)),
                    ..dialog_styles(appearance)
                },
            );
        if !cfg!(feature = "warp_lite") {
            dialog = dialog.with_child(dont_show_again_checkbox);
        }
        let dialog = Container::new(
            dialog
                .with_bottom_row_child(cancel_button)
                .with_bottom_row_child(close_session_button)
                .build()
                .finish(),
        )
        .with_margin_top(35.)
        .finish();
        #[cfg(feature = "integration_tests")]
        let dialog = warpui::elements::SavePosition::new(dialog, "close-session-dialog").finish();

        // Stack needed so that dialog can get bounds information,
        // specifically to ensure no overlap with the window's traffic lights
        let mut stack = Stack::new();
        stack.add_positioned_child(
            dialog,
            OffsetPositioning::offset_from_parent(
                vec2f(0., 0.),
                ParentOffsetBounds::WindowByPosition,
                ParentAnchor::Center,
                ChildAnchor::Center,
            ),
        );

        // This blurs the background and makes it uninteractable
        Container::new(Align::new(stack.finish()).finish())
            .with_background_color(Fill::blur().into())
            .with_corner_radius(app.windows().window_corner_radius())
            .finish()
    }
}

pub enum CloseSessionConfirmationEvent {
    CloseSession {
        dont_show_again: bool,
        open_confirmation_source: OpenDialogSource,
    },
    Cancel,
}

#[derive(Debug)]
pub enum CloseSessionConfirmationAction {
    CloseSession { dont_show_again: bool },
    Cancel,
    ToggleDontShowAgain,
}

impl TypedActionView for CloseSessionConfirmationDialog {
    type Action = CloseSessionConfirmationAction;

    fn handle_action(
        &mut self,
        action: &CloseSessionConfirmationAction,
        ctx: &mut ViewContext<Self>,
    ) {
        match action {
            CloseSessionConfirmationAction::CloseSession { dont_show_again } => {
                let Some(open_confirmation_source) = self.open_confirmation_source.take() else {
                    // Should not be possible.
                    report_error!(
                        "Close session button pressed with no open confirmation dialog source"
                    );
                    return;
                };
                ctx.emit(CloseSessionConfirmationEvent::CloseSession {
                    dont_show_again: !cfg!(feature = "warp_lite") && *dont_show_again,
                    open_confirmation_source,
                });
            }
            CloseSessionConfirmationAction::Cancel => {
                self.open_confirmation_source = None;
                ctx.emit(CloseSessionConfirmationEvent::Cancel);
                self.dont_show_again = false;
            }
            CloseSessionConfirmationAction::ToggleDontShowAgain => {
                if !cfg!(feature = "warp_lite") {
                    self.dont_show_again = !self.dont_show_again;
                    ctx.notify();
                }
            }
        }
    }
}
