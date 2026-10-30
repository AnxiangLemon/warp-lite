//! Lite stub for the inline plan menu (no AIDocumentModel).

use crate::ai::document::{AIDocumentId, AIDocumentVersion};
use warpui::elements::{Element, Empty, ParentElement};
use warpui::{AppContext, Entity, ModelHandle, View, ViewContext, ViewHandle};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::blocklist::agent_view::AgentViewController;
use crate::terminal::input::buffer_model::InputBufferModel;
use crate::terminal::input::inline_menu::InlineMenuPositioner;
use crate::terminal::input::suggestions_mode_model::InputSuggestionsModeModel;

#[derive(Clone, Debug)]
pub enum InlinePlanMenuEvent {
    OpenPlan {
        document_id: AIDocumentId,
        document_version: AIDocumentVersion,
    },
    Dismissed,
}

pub struct InlinePlanMenuView;

impl InlinePlanMenuView {
    pub fn new(
        _conversation_id: AIConversationId,
        _suggestions_mode_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _buffer_model: &ModelHandle<InputBufferModel>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn select_up(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept_selected_item(&mut self, _ctx: &mut ViewContext<Self>) -> bool {
        false
    }
}

impl Entity for InlinePlanMenuView {
    type Event = InlinePlanMenuEvent;
}

impl View for InlinePlanMenuView {
    fn ui_name() -> &'static str {
        "InlinePlanMenuViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
