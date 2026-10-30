//! Lite stub for the queued-prompts panel.

use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, EntityId, ModelHandle, TypedActionView, View, ViewContext, ViewHandle};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::blocklist::QueuedQueryId;
use crate::ai::blocklist::block::cli_controller::CLISubagentController;
use crate::editor::EditorView;
use crate::terminal::input::suggestions_mode_model::InputSuggestionsModeModel;

pub struct QueuedPromptsPanelView;

impl QueuedPromptsPanelView {
    pub fn new(
        _terminal_view_id: EntityId,
        _suggestions_mode_model: ModelHandle<InputSuggestionsModeModel>,
        _cli_subagent_controller: ModelHandle<CLISubagentController>,
        _host_editor: ViewHandle<EditorView>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn set_can_send_prompt(&mut self, _can_send: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn enter_sends_queued_prompt(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn is_inline_edit_editor_focused(&self, _app: &AppContext) -> bool {
        false
    }

    pub(crate) fn commit_edit(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for QueuedPromptsPanelView {
    type Event = QueuedPromptsPanelEvent;
}

impl View for QueuedPromptsPanelView {
    fn ui_name() -> &'static str {
        "QueuedPromptsPanelViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for QueuedPromptsPanelView {
    type Action = QueuedPromptsPanelAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

#[derive(Debug, Clone)]
pub enum QueuedPromptsPanelAction {}

#[derive(Debug, Clone)]
pub enum QueuedPromptsPanelEvent {
    SendNow {
        conversation_id: AIConversationId,
        query_id: QueuedQueryId,
        text: String,
        is_command: bool,
    },
    RowDeleted,
    EditEnded,
}
