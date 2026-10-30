use warpui::keymap::Keystroke;

pub trait AITip: Clone {
    fn keystroke(&self, _app: &warpui::AppContext) -> Option<Keystroke> {
        None
    }

    fn link(&self) -> Option<String> {
        None
    }

    fn description(&self) -> &str {
        ""
    }
}

pub struct AgentTip;

pub mod popup {
    pub fn init(_app: &mut warpui::AppContext) {}
}

use warpui::{Entity, ModelContext, SingletonEntity};

pub struct AITipModel<T: 'static> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: 'static> AITipModel<T> {
    pub fn new_for_agent_tips(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn revalidate_tips(&mut self, _ctx: &mut ModelContext<Self>) {}
}

impl<T: 'static> Entity for AITipModel<T> {
    type Event = ();
}

impl<T: 'static> SingletonEntity for AITipModel<T> {}
