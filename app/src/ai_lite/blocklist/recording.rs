use warpui::{Entity, ModelContext, SingletonEntity};

pub struct RecordingController;

impl RecordingController {
    pub fn new() -> Self {
        Self
    }
}

impl Entity for RecordingController {
    type Event = ();
}

impl SingletonEntity for RecordingController {}

pub mod blocklist {
    pub use super::RecordingController;
}
