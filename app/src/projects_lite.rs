use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::SyncSender;

use warpui::{Entity, ModelContext, SingletonEntity};

use crate::persistence::ModelEvent;
use crate::persistence::model::Project;

#[derive(Debug)]
pub enum ProjectEvent {
    Added {
        path: PathBuf,
    },
    Removed { path: PathBuf },
    Updated { path: PathBuf },
}

pub struct ProjectManagementModel {
    projects: HashMap<PathBuf, Project>,
}

impl Entity for ProjectManagementModel {
    type Event = ProjectEvent;
}

impl SingletonEntity for ProjectManagementModel {}

impl ProjectManagementModel {
    pub fn new(
        _persisted_projects: Vec<Project>,
        _model_event_sender: Option<SyncSender<ModelEvent>>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    pub fn upsert_project(&mut self, _path: PathBuf, _ctx: &mut ModelContext<Self>) {}

    pub fn all_projects(&self) -> impl Iterator<Item = &Project> {
        self.projects.values()
    }
}
