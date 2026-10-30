use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Clone, Debug)]
pub enum ConnectedSelfHostedWorkersEvent {}

pub struct ConnectedSelfHostedWorkersModel;

impl ConnectedSelfHostedWorkersModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for ConnectedSelfHostedWorkersModel {
    type Event = ConnectedSelfHostedWorkersEvent;
}

impl SingletonEntity for ConnectedSelfHostedWorkersModel {}
