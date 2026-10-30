use warpui::{Entity, ModelContext, SingletonEntity};

use crate::cloud_object::Owner;
use crate::server::ids::SyncId;
use crate::{PaneViewLocator, WindowId};

pub struct EnvVarCollectionManager {}

#[derive(Debug, Clone)]
pub enum EnvVarCollectionSource {
    Existing(SyncId),
    New {
        title: Option<String>,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
    },
}

impl EnvVarCollectionManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {}
    }

    pub fn find_pane(
        &self,
        _source: &EnvVarCollectionSource,
    ) -> Option<(WindowId, PaneViewLocator)> {
        None
    }

    pub fn reset(&mut self) {}
}

impl Entity for EnvVarCollectionManager {
    type Event = ();
}

impl SingletonEntity for EnvVarCollectionManager {}
