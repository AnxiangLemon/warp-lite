//! Lite stub for the terminal zero-state block.

use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::ai::blocklist::agent_view::AgentViewController;
use crate::terminal::model_events::ModelEventDispatcher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalViewZeroStateAction {
    ToggleNLD,
    Dismiss,
}

pub struct TerminalViewZeroStateBlock;

impl TerminalViewZeroStateBlock {
    pub fn new(
        _agent_view_controller: &ModelHandle<AgentViewController>,
        _model_events_dispatcher: &ModelHandle<ModelEventDispatcher>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for TerminalViewZeroStateBlock {
    type Event = TerminalViewZeroStateAction;
}

impl View for TerminalViewZeroStateBlock {
    fn ui_name() -> &'static str {
        "TerminalViewZeroStateBlockLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for TerminalViewZeroStateBlock {
    type Action = TerminalViewZeroStateAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
