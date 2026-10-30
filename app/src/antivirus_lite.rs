use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Debug, Clone)]
pub struct AntivirusInfo;

impl AntivirusInfo {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn get(&self) -> Option<&str> {
        None
    }
}

pub enum AntivirusInfoEvent {}

impl Entity for AntivirusInfo {
    type Event = AntivirusInfoEvent;
}

impl SingletonEntity for AntivirusInfo {}
