use ::settings::Setting as _;
use warp_util::path::ShellFamily;
use warpui::elements::{ChildView, Flex, ParentElement, SavePosition};
use warpui::{AppContext, Element, ModelHandle, SingletonEntity, ViewContext};

use super::{Event, Input, InputPrefixMode};
use crate::completer::SessionContext;
use crate::settings::InputModeSettings;
use crate::terminal::block_list_viewport::InputMode;
use crate::terminal::session_types::InputType;
use crate::terminal::view::ambient_agent::AmbientAgentViewModel;
use crate::view_components::ToastFlavor;

impl Input {
    pub(super) fn should_restore_buffer_on_inline_menu_dismiss(
        &self,
        ctx: &ViewContext<Self>,
    ) -> bool {
        let _ = ctx;
        true
    }

    pub(super) fn can_attach_on_filepaths_paste_or_dragdrop(
        &self,
        ctx: &mut ViewContext<Self>,
    ) -> bool {
        let _ = ctx;
        false
    }

    pub fn handle_pasted_or_dragdropped_image_filepaths(
        &mut self,
        image_filepaths: Vec<String>,
        ctx: &mut ViewContext<Self>,
    ) -> usize {
        let _ = (image_filepaths, ctx);
        0
    }

    pub(super) fn render_native_input_with_inline_history(
        &self,
        input: Box<dyn Element>,
        ctx: &AppContext,
    ) -> Box<dyn Element> {
        let positioner = self.inline_terminal_menu_positioner.as_ref(ctx);
        let show_history = self
            .suggestions_mode_model
            .as_ref(ctx)
            .is_inline_history_menu()
            && !positioner.should_hide_inline_menu_for_pane_size(ctx);
        let history_below = match *InputModeSettings::as_ref(ctx).input_mode.value() {
            InputMode::PinnedToBottom => false,
            InputMode::PinnedToTop => true,
            InputMode::Waterfall => positioner.should_render_inline_menu_below_input(),
        };
        let mut column = Flex::column();
        if show_history && !history_below {
            column.add_child(ChildView::new(&self.inline_history_menu_view).finish());
        }
        column.add_child(input);
        if show_history && history_below {
            column.add_child(ChildView::new(&self.inline_history_menu_view).finish());
        }
        SavePosition::new(column.finish(), &self.save_position_id()).finish()
    }

    pub(super) fn show_agent_runtime_unavailable(&self, ctx: &mut ViewContext<Self>) {
        ctx.emit(Event::ShowToast {
            message: crate::lite_i18n::choose(
                "Agent runtime is unavailable in this build",
                "此版本不提供智能体运行环境",
            )
            .to_owned(),
            flavor: ToastFlavor::Error,
        });
    }

    pub fn update_image_context_options(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub fn check_and_update_ai_context_menu_disabled_state(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(super) fn check_slash_menu_disabled_state(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(super) fn update_ai_context_menu(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(super) fn has_queued_command_in_flight(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub(super) fn is_editing_queued_prompt(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub fn input_type(&self, ctx: &AppContext) -> InputType {
        let _ = ctx;
        InputType::Shell
    }

    pub(super) fn is_locked_in_shell_mode(&self, ctx: &ViewContext<Self>) -> bool {
        let _ = ctx;
        true
    }

    pub(super) fn is_locked_in_ai_mode(&self, ctx: &ViewContext<Self>) -> bool {
        let _ = ctx;
        false
    }

    pub fn is_cloud_mode_input_v2_composing(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub(crate) fn is_queued_prompt_inline_editor_focused(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub(super) fn ambient_agent_view_model(&self) -> Option<&ModelHandle<AmbientAgentViewModel>> {
        None
    }

    pub(super) fn prefix_mode(&self, ctx: &AppContext) -> InputPrefixMode {
        let _ = ctx;
        InputPrefixMode::None
    }

    pub(super) fn should_enable_ai_context(
        &self,
        buffer_text: &str,
        cursor_position: usize,
        is_alias_expansion_enabled: bool,
        session_context: Option<&SessionContext>,
        shell_family: ShellFamily,
        ctx: &AppContext,
    ) -> bool {
        let _ = (
            buffer_text,
            cursor_position,
            is_alias_expansion_enabled,
            session_context,
            shell_family,
            ctx,
        );
        false
    }
}
