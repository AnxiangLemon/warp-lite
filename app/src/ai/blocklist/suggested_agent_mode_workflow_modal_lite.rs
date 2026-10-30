//! Lite stub for suggested agent mode workflow modal (no WorkflowView / Warp Drive).

use std::collections::HashMap;
use std::sync::Arc;

use warpui::{AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle};

use crate::ai::agent::SuggestedAgentModeWorkflow;
use crate::server::ids::SyncId;
use crate::workflows::{WorkflowSelectionSource, WorkflowSource, WorkflowType};

#[derive(Debug, Clone, Default)]
pub struct SuggestedAgentModeWorkflowModal {}

#[derive(Debug, Clone)]
pub struct SuggestedAgentModeWorkflowAndId {
    pub workflow: SuggestedAgentModeWorkflow,
    pub sync_id: SyncId,
}

#[derive(Debug, Clone)]
pub enum SuggestedAgentModeWorkflowModalAction {
    Cancel,
}

#[derive(Debug, Clone)]
pub enum SuggestedAgentModeWorkflowModalEvent {
    Close,
    WorkflowCreated,
    RunWorkflow {
        workflow: Arc<WorkflowType>,
        source: Box<WorkflowSource>,
        argument_override: Option<HashMap<String, String>>,
        workflow_selection_source: WorkflowSelectionSource,
    },
}

pub fn init(_app: &mut AppContext) {}

impl SuggestedAgentModeWorkflowModal {
    pub fn close(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn open_workflow(
        &mut self,
        _workflow_and_id: &SuggestedAgentModeWorkflowAndId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl Entity for SuggestedAgentModeWorkflowModal {
    type Event = SuggestedAgentModeWorkflowModalEvent;
}

impl View for SuggestedAgentModeWorkflowModal {
    fn ui_name() -> &'static str {
        "SuggestedAgentModeWorkflowModalLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
        Box::new(warpui::elements::Empty::new())
    }
}

impl TypedActionView for SuggestedAgentModeWorkflowModal {
    type Action = SuggestedAgentModeWorkflowModalAction;
}
