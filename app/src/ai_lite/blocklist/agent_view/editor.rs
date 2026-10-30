use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelContext, View, ViewContext};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentToolbarEditorMode {
    #[default]
    Unknown,
}

#[derive(Clone, Debug)]
pub enum AgentToolbarEditorEvent {}

pub struct AgentToolbarEditorModal;

impl Entity for AgentToolbarEditorModal {
    type Event = AgentToolbarEditorEvent;
}

impl View for AgentToolbarEditorModal {
    fn ui_name() -> &'static str {
        "AgentToolbarEditorModalLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

pub fn init(_app: &mut AppContext) {}
