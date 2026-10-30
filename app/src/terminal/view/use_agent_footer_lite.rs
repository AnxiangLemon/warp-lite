//! Lite stub for the "Use agent" footer and related TerminalView helpers.

use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::elements::{Element, Empty};
use warpui::{
    AppContext, Entity, EntityId, ModelHandle, TypedActionView, View, ViewContext, ViewHandle,
};

use crate::ai::blocklist::agent_view::agent_input_footer::AgentInputFooter;
use crate::terminal::TerminalModel;
use crate::terminal::TerminalView;
use crate::terminal::cli_agent_sessions::{
    CLIAgentInputEntrypoint, CLIAgentRichInputCloseReason,
};
use crate::terminal::model_events::ModelEventDispatcher;
use crate::terminal::CLIAgent;

pub struct UseAgentToolbar;

impl UseAgentToolbar {
    pub(crate) fn new(
        _terminal_view_id: EntityId,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _model_event_dispatcher: &ModelHandle<ModelEventDispatcher>,
        _agent_input_footer: ViewHandle<AgentInputFooter>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn did_user_dismiss(&self) -> bool {
        false
    }

    #[cfg(feature = "voice_input")]
    pub fn has_cli_agent(&self, _app: &AppContext) -> bool {
        false
    }

    pub(in crate::terminal) fn is_warpify_active(&self, _app: &AppContext) -> bool {
        false
    }

    pub(in crate::terminal) fn notify_and_notify_children(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal) fn show_warpify(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(in crate::terminal) fn clear_warpify(&mut self, _ctx: &mut ViewContext<Self>) {}
}

pub enum UseAgentToolbarEvent {
    Dismiss,
}

impl Entity for UseAgentToolbar {
    type Event = UseAgentToolbarEvent;
}

impl View for UseAgentToolbar {
    fn ui_name() -> &'static str {
        "UseAgentToolbarLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Debug, Clone)]
pub enum UseAgentToolbarAction {
    Dismiss { permanently: bool },
}

impl TypedActionView for UseAgentToolbar {
    type Action = UseAgentToolbarAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl TerminalView {
    pub(super) fn register_subscriptions_for_use_agent_footer(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn has_active_cli_agent_input_session(&self, _app: &AppContext) -> bool {
        false
    }

    pub(super) fn should_render_use_agent_footer(
        &self,
        _model: &TerminalModel,
        _app: &AppContext,
    ) -> bool {
        false
    }

    pub(super) fn detect_cli_agent_from_model(
        &self,
        _model: &TerminalModel,
        _ctx: &AppContext,
    ) -> Option<(CLIAgent, Option<String>)> {
        None
    }

    pub(super) fn is_running_warp_tui(
        &self,
        _model: &TerminalModel,
        _ctx: &AppContext,
    ) -> bool {
        false
    }

    pub(super) fn tag_in_agent_for_user_long_running_command(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn tag_out_agent_for_user_long_running_command(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn maybe_show_use_agent_footer_in_blocklist(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn hide_use_agent_footer_in_blocklist(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(in crate::terminal) fn close_cli_agent_rich_input(
        &mut self,
        _reason: CLIAgentRichInputCloseReason,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal) fn close_cli_agent_rich_input_and_disable_auto_toggle(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn submit_cli_agent_rich_input(
        &mut self,
        _text: String,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn paste_dropped_images_to_cli_agent(
        &mut self,
        _image_filepaths: Vec<String>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal) fn open_cli_agent_rich_input(
        &mut self,
        _entrypoint: CLIAgentInputEntrypoint,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(crate) fn submit_text_to_cli_agent_pty(
        &mut self,
        _text: String,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}
