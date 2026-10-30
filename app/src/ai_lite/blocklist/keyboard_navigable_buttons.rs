use warpui::elements::{Element, Empty};
use warpui::ui_components::button::Button;
use warpui::{Action, AppContext, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

#[derive(Debug, Clone)]
pub enum KeyboardNavigableButtonsAction {}

pub struct KeyboardNavigableButtonBuilder;

impl KeyboardNavigableButtonBuilder {
    pub fn new(
        _button_builder: impl Fn(bool, &AppContext) -> Button + 'static,
        _on_selected: impl Fn(&mut ViewContext<KeyboardNavigableButtons>) + 'static,
    ) -> Self {
        Self
    }
}

pub fn simple_navigation_button<A: Action + Clone + 'static>(
    _text_label: String,
    _mouse_state: warpui::elements::MouseStateHandle,
    _action: A,
    _disabled: bool,
) -> KeyboardNavigableButtonBuilder {
    KeyboardNavigableButtonBuilder
}

pub fn rich_navigation_button(
    _text_label: String,
    _description: String,
    _mouse_state: warpui::elements::MouseStateHandle,
    _action: impl Action + Clone + 'static,
    _disabled: bool,
) -> KeyboardNavigableButtonBuilder {
    KeyboardNavigableButtonBuilder
}

pub struct KeyboardNavigableButtons;

impl Entity for KeyboardNavigableButtons {
    type Event = ();
}

impl View for KeyboardNavigableButtons {
    fn ui_name() -> &'static str {
        "KeyboardNavigableButtonsLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for KeyboardNavigableButtons {
    type Action = KeyboardNavigableButtonsAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
