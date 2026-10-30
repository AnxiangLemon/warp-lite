pub use crate::ai::cloud_environments::scheduled::{
    CloudScheduledAmbientAgentModel, ScheduledAmbientAgent,
};

use warpui::{Entity, ModelContext, SingletonEntity};

pub struct ScheduledAgentManager;

impl ScheduledAgentManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for ScheduledAgentManager {
    type Event = ();
}

impl SingletonEntity for ScheduledAgentManager {}
