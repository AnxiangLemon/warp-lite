use warpui::{Entity, SingletonEntity};

pub struct RestoredAgentConversations;

impl RestoredAgentConversations {
    pub fn new() -> Self {
        Self
    }
}

impl Entity for RestoredAgentConversations {
    type Event = ();
}

impl SingletonEntity for RestoredAgentConversations {}
