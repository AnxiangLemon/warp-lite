use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

pub fn init(_app: &mut AppContext) {}

#[derive(Debug)]
pub enum RewardAction {
    OpenThemePicker,
}

pub enum RewardEvent {
    OpenThemePicker,
}

#[derive(Clone, Copy)]
pub enum RewardKind {
    SentReferralTheme,
    ReceivedReferralTheme,
}

pub struct RewardView {}

impl Default for RewardView {
    fn default() -> Self {
        Self::new()
    }
}

impl RewardView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update_reward_kind(&mut self, _kind: RewardKind, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for RewardView {
    type Event = RewardEvent;
}

impl View for RewardView {
    fn ui_name() -> &'static str {
        "RewardView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for RewardView {
    type Action = RewardAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
