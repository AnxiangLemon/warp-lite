use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, View, ViewContext};

#[derive(Clone, Debug)]
pub enum ConversationDetailsPanelEvent {}

pub struct ConversationDetailsData;

pub struct ConversationDetailsPanel;

impl ConversationDetailsPanel {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Entity for ConversationDetailsPanel {
    type Event = ConversationDetailsPanelEvent;
}

impl View for ConversationDetailsPanel {
    fn ui_name() -> &'static str {
        "ConversationDetailsPanelLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

pub fn init(_app: &mut AppContext) {}
