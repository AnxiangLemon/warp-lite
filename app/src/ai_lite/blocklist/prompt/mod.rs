pub mod prompt_alert;

pub struct PromptIconButtonTheme;

pub mod plan_and_todo_list {
    use warpui::elements::{Element, Empty};
    use warpui::{AppContext, Entity, View, ViewContext};

    #[derive(Clone, Debug)]
    pub enum PlanAndTodoListEvent {}

    pub struct PlanAndTodoListView;

    impl Entity for PlanAndTodoListView {
        type Event = PlanAndTodoListEvent;
    }

    impl View for PlanAndTodoListView {
        fn ui_name() -> &'static str {
            "PlanAndTodoListViewLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}
