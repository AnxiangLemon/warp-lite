use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutlineStatus {
    #[default]
    Unknown,
}

#[derive(Clone, Debug)]
pub enum RepoOutlinesEvent {}

pub struct RepoOutlines;

impl RepoOutlines {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn new_with_indexing_enabled(_enabled: bool, _ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for RepoOutlines {
    type Event = RepoOutlinesEvent;
}

impl SingletonEntity for RepoOutlines {}
