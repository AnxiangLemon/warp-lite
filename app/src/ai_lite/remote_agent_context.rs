use warpui::{Entity, ModelContext, SingletonEntity};

pub struct RemoteAgentContext;

impl RemoteAgentContext {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for RemoteAgentContext {
    type Event = ();
}

impl SingletonEntity for RemoteAgentContext {}
