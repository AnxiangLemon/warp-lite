use warpui::elements::{Element, Empty};
use warpui::ui_components::text::Span;
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

pub struct ToggleableItemBuilder<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> ToggleableItemBuilder<T> {
    pub fn new(
        _label_fn: impl Fn(&T, &AppContext) -> Span + 'static,
        _is_selected_fn: impl Fn(&T) -> bool + 'static,
    ) -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ToggleableItemsAction {}

#[derive(Debug, Clone)]
pub enum ToggleableItemsEvent {
    SelectionChanged,
    SubmitRequested,
}

pub struct ToggleableItemsView<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> ToggleableItemsView<T> {
    pub fn new(_items: Vec<T>, _builder: ToggleableItemBuilder<T>) -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: 'static> Entity for ToggleableItemsView<T> {
    type Event = ToggleableItemsEvent;
}

impl<T: 'static> View for ToggleableItemsView<T> {
    fn ui_name() -> &'static str {
        "ToggleableItemsViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl<T: 'static> TypedActionView for ToggleableItemsView<T> {
    type Action = ToggleableItemsAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
