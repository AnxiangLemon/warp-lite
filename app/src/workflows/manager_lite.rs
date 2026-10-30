use warpui::{Entity, ModelContext, SingletonEntity};

use super::workflow::Workflow;
use crate::cloud_object::Owner;
use crate::server::ids::SyncId;
use crate::workflows::WorkflowViewMode;
use crate::{PaneViewLocator, WindowId};

pub struct WorkflowManager {}

#[derive(Debug, Clone)]
pub enum WorkflowOpenSource {
    Existing(SyncId),
    New {
        title: Option<String>,
        content: Option<String>,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
        is_for_agent_mode: bool,
    },
    NewFromWorkflow {
        workflow: Box<Workflow>,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
    },
}

impl WorkflowManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {}
    }

    pub fn find_pane(
        &self,
        _source: &WorkflowOpenSource,
    ) -> Option<(WindowId, PaneViewLocator)> {
        None
    }

    pub fn reset(&mut self) {}
}

impl Entity for WorkflowManager {
    type Event = ();
}

impl SingletonEntity for WorkflowManager {}
