use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, View, ViewContext};

    #[derive(Clone, Debug)]
    pub enum PromptAlertEvent {}

    pub struct PromptAlertState;

    pub struct PromptAlertView;

impl Entity for PromptAlertView {
    type Event = PromptAlertEvent;
}

impl View for PromptAlertView {
    fn ui_name() -> &'static str {
        "PromptAlertViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
