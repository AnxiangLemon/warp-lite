use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, View, ViewContext};

pub struct Artifact;

pub struct ArtifactButtonsRow;

impl Entity for ArtifactButtonsRow {
    type Event = ArtifactButtonsRowEvent;
}

impl View for ArtifactButtonsRow {
    fn ui_name() -> &'static str {
        "ArtifactButtonsRowLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

#[derive(Clone, Debug)]
pub enum ArtifactButtonsRowEvent {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ArtifactType {
    #[default]
    Unknown,
}

pub fn open_screenshot_lightbox<V: View>(_artifact_uids: Vec<String>, _ctx: &mut ViewContext<V>) {}

pub fn download_file_artifact<V: View>(_artifact_uid: String, _ctx: &mut ViewContext<V>) {}
