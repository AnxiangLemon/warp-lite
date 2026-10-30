use warpui::{Entity, SingletonEntity};

#[derive(Clone, Debug)]
pub enum GitHubAuthEvent {}

pub struct GitHubAuthNotifier;

impl GitHubAuthNotifier {
    pub fn new() -> Self {
        Self
    }
}

impl Entity for GitHubAuthNotifier {
    type Event = GitHubAuthEvent;
}

impl SingletonEntity for GitHubAuthNotifier {}
