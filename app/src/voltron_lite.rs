use std::path::PathBuf;

use pathfinder_geometry::vector::Vector2F;
use vec1::Vec1;
use warpui::elements::Empty;
use warpui::keymap::Context;
use warpui::{
    AppContext, Element, Entity, TypedActionView, View, ViewContext, ViewHandle,
};

use crate::editor::Event as EditorEvent;
use crate::terminal::input::MenuPositioning;
use crate::util::bindings::CustomAction;
use crate::workflows::categories::CategoriesView;

pub fn init(_app: &mut AppContext) {}

#[derive(Clone)]
pub struct VoltronFeatureView {
    pub name: VoltronItem,
    pub feature_view_handle: VoltronFeatureViewHandle,
}

#[derive(Clone)]
pub enum VoltronFeatureViewHandle {
    Workflows(ViewHandle<CategoriesView>),
}

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum VoltronItem {
    AiCommands,
    Workflows,
    History,
}

impl VoltronItem {
    pub fn as_str(&self) -> &'static str {
        match self {
            VoltronItem::AiCommands => "A.I. Command Search",
            VoltronItem::Workflows => "Workflows",
            VoltronItem::History => "History Search",
        }
    }
}

#[derive(Clone, Default)]
pub struct VoltronMetadata {
    pub active_session_path_if_local: Option<PathBuf>,
    pub starting_editor_text: Option<String>,
    pub keymap_context: Context,
    pub menu_positioning: MenuPositioning,
}

pub trait VoltronFeatureViewMeta {
    fn editor_placeholder_text(&self) -> &'static str;

    fn handle_editor_event(
        &mut self,
        event: &EditorEvent,
        current_editor_text: &str,
        ctx: &mut ViewContext<Self>,
    );

    fn on_load(&mut self, _metadata: VoltronMetadata, _ctx: &mut ViewContext<Self>) {}

    fn custom_action() -> Option<CustomAction>;

    fn close(&mut self, _ctx: &mut ViewContext<Self>) {}
}

pub struct Voltron;

#[derive(Debug, Clone)]
pub enum VoltronEvent {
    Close,
}

#[derive(Debug, Clone)]
pub enum VoltronAction {}

impl VoltronFeatureView {
    pub fn new(name: VoltronItem, feature_view_handle: VoltronFeatureViewHandle) -> Self {
        Self {
            name,
            feature_view_handle,
        }
    }
}

impl Voltron {
    pub fn new(_features: Vec1<VoltronFeatureView>, _ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn select_and_refresh_by_name(
        &mut self,
        _feature_name: VoltronItem,
        _metadata: VoltronMetadata,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn close(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for Voltron {
    type Event = VoltronEvent;
}

impl TypedActionView for Voltron {
    type Action = VoltronAction;
}

impl View for Voltron {
    fn ui_name() -> &'static str {
        "Voltron"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Box::new(Empty::new())
    }
}

pub type ParentAnchor = warpui::elements::ParentAnchor;
pub type ChildAnchor = warpui::elements::ChildAnchor;

pub fn voltron_offset(_menu_positioning: MenuPositioning) -> Vector2F {
    Vector2F::zero()
}
