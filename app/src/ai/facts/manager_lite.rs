use std::collections::HashMap;

use warpui::{Entity, EntityId, ModelContext, SingletonEntity, ViewHandle, WindowId};

use crate::PaneViewLocator;
use crate::ai::facts::AIFactView;

#[derive(Default)]
pub struct AIFactManager {
    views: HashMap<WindowId, ViewHandle<AIFactView>>,
}

impl AIFactManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ai_fact_view(&self, window_id: WindowId) -> ViewHandle<AIFactView> {
        self.views
            .get(&window_id)
            .expect("Window should have corresponding AI fact view")
            .clone()
    }

    pub fn register_view(&mut self, window_id: WindowId, view: ViewHandle<AIFactView>) {
        self.views.insert(window_id, view);
    }

    pub fn find_pane(&self, _window_id: WindowId) -> Option<PaneViewLocator> {
        None
    }

    pub fn register_pane(
        &mut self,
        _pane_group_id: EntityId,
        _window_id: WindowId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn deregister_pane(&mut self, _window_id: &WindowId, _ctx: &mut ModelContext<Self>) {}
}

impl Entity for AIFactManager {
    type Event = ();
}

impl SingletonEntity for AIFactManager {}
