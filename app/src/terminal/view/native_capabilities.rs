use warpui::{AppContext, ViewContext};

use super::TerminalView;
use crate::menu::MenuItem;
use crate::terminal::TerminalModel;
use crate::terminal::session_types::{
    AIAgentExchangeId, AIConversationId, InputConfig, InputType, TaskId,
};
use crate::terminal::view::TerminalAction;

impl TerminalView {
    pub(super) fn thread_start_exchange_id(
        conversation_id: &AIConversationId,
        ctx: &AppContext,
    ) -> Option<AIAgentExchangeId> {
        let _ = (conversation_id, ctx);
        None
    }
    pub(super) fn can_show_conversation_details_ui_from_model(
        &self,
        model: &TerminalModel,
        ctx: &AppContext,
    ) -> bool {
        let _ = (model, ctx);
        false
    }
    pub(super) fn should_render_legacy_ambient_agent_loading_footer(
        &self,
        model: &TerminalModel,
        ctx: &AppContext,
    ) -> bool {
        let _ = (model, ctx);
        false
    }

    pub(super) fn abort_prompt_and_code_suggestions(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(super) fn has_active_cli_agent_input_session(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub(super) fn should_render_use_agent_footer(
        &self,
        model: &TerminalModel,
        ctx: &AppContext,
    ) -> bool {
        let _ = (model, ctx);
        false
    }

    pub(super) fn detect_cli_agent_from_model(
        &self,
        model: &TerminalModel,
        ctx: &AppContext,
    ) -> Option<(crate::terminal::CLIAgent, Option<String>)> {
        let _ = (model, ctx);
        None
    }

    pub(super) fn is_running_warp_tui(&self, model: &TerminalModel, ctx: &AppContext) -> bool {
        let _ = (model, ctx);
        false
    }

    pub(super) fn maybe_show_use_agent_footer_in_blocklist(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(super) fn hide_use_agent_footer_in_blocklist(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(in crate::terminal::view) fn blocks_cloud_followups_for_ambient_agent_session_from_model(
        &self,
        model: &TerminalModel,
        ctx: &AppContext,
    ) -> bool {
        let _ = (model, ctx);
        false
    }

    pub fn session_sharing_context_menu_items(
        &self,
        model: &TerminalModel,
        is_share_session_disabled: bool,
        has_session_link: bool,
    ) -> Vec<MenuItem<TerminalAction>> {
        let _ = (model, is_share_session_disabled, has_session_link);
        Vec::new()
    }

    pub(crate) fn is_viewer_driven_sizing_eligible(
        &self,
        is_sharer: bool,
        ctx: &ViewContext<Self>,
    ) -> bool {
        let _ = (is_sharer, ctx);
        false
    }

    pub(crate) fn restore_pty_to_sharer_size(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(in crate::terminal::view) fn force_report_viewer_terminal_size(
        &mut self,
        ctx: &mut ViewContext<Self>,
    ) {
        let _ = ctx;
    }

    pub(super) fn sync_pending_context_block_ids(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(crate) fn on_queued_command_finished(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(crate) fn clear_queued_command_in_flight(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub(crate) fn has_queued_command_in_flight(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub fn active_cli_agent(&self, ctx: &AppContext) -> Option<crate::terminal::CLIAgent> {
        let _ = ctx;
        None
    }

    pub fn is_cli_agent_rich_input_open(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub(super) fn has_active_cli_agent_session(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub(super) fn should_hide_cli_agent_cursor_cell(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub fn input_config(&self, ctx: &AppContext) -> InputConfig {
        let _ = ctx;
        InputConfig {
            input_type: InputType::Shell,
            is_locked: true,
        }
    }

    pub fn active_conversation_id(&self, ctx: &AppContext) -> Option<AIConversationId> {
        let _ = ctx;
        None
    }

    pub fn active_conversation_task_id(&self, ctx: &AppContext) -> Option<TaskId> {
        let _ = ctx;
        None
    }

    pub(super) fn should_ctrl_c_exit_agent_view(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }

    pub(super) fn should_use_agent_transcript_navigation(&self, ctx: &AppContext) -> bool {
        let _ = ctx;
        false
    }
}
