pub mod popup {
    use warpui::elements::{Element, Empty};
    use warpui::{AppContext, Entity, ModelHandle, View, ViewContext};

    #[derive(Debug, Clone)]
    pub enum AgentTodosPopupEvent {}

    pub struct AgentTodosPopupView;

    impl AgentTodosPopupView {
        pub fn new(
            _ai_context_model: ModelHandle<crate::ai::blocklist::BlocklistAIContextModel>,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self
        }
    }

    impl Entity for AgentTodosPopupView {
        type Event = AgentTodosPopupEvent;
    }

    impl View for AgentTodosPopupView {
        fn ui_name() -> &'static str {
            "AgentTodosPopupViewLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    pub fn init(_app: &mut AppContext) {}
}

#[derive(Clone, Debug, Default)]
pub struct AIAgentTodoList;
