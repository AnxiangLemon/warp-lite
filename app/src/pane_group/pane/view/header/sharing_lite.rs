//! Lite stub for pane-header sharing controls (no Warp Drive sharing dialog).

use warpui::elements::{Empty, MouseStateHandle};
use warpui::{AppContext, Element, Entity, TypedActionView, View, ViewContext, ViewHandle};

use super::{Event, OpenOverlay, PaneHeader, PaneHeaderAction};
use crate::cloud_object::{ContentEditability, ShareableObject};
use crate::pane_group::BackingView;
use crate::server::telemetry::SharingDialogSource;
use warp_core::ui::appearance::Appearance;
use warpui::elements::ParentElement;

pub struct SharingDialogLite;

#[derive(Clone, Debug)]
pub enum SharingDialogEvent {
    Close,
}

impl SharingDialogLite {
    pub fn new(_initial_target: Option<ShareableObject>, _ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn set_target(&mut self, _target: Option<ShareableObject>, _ctx: &mut ViewContext<Self>) {}

    pub fn has_target(&self) -> bool {
        false
    }

    pub fn has_shared_session_target(&self) -> bool {
        false
    }

    pub fn editability<C: warpui::ViewAsRef>(&self, _ctx: &C) -> ContentEditability {
        ContentEditability::Editable
    }

    pub fn copy_link(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn report_open(&mut self, _source: SharingDialogSource, _ctx: &mut ViewContext<Self>) {}

    pub fn show_qr_code(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn is_unsharable_conversation(&self, _app: &AppContext) -> bool {
        false
    }
}

impl Entity for SharingDialogLite {
    type Event = SharingDialogEvent;
}

impl View for SharingDialogLite {
    fn ui_name() -> &'static str {
        "SharingDialogLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for SharingDialogLite {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

pub struct SharedPaneContent {
    sharing_dialog: ViewHandle<SharingDialogLite>,
    primary_button_handle: MouseStateHandle,
    view_only_icon_handle: MouseStateHandle,
}

impl SharedPaneContent {
    pub fn new<P: BackingView>(ctx: &mut ViewContext<PaneHeader<P>>) -> Self {
        let sharing_dialog = ctx.add_typed_action_view(|ctx| SharingDialogLite::new(None, ctx));
        ctx.subscribe_to_view(&sharing_dialog, move |me, _, event, ctx| {
            me.handle_sharing_dialog_event(event, ctx);
        });
        Self {
            sharing_dialog,
            primary_button_handle: Default::default(),
            view_only_icon_handle: Default::default(),
        }
    }
}

impl<P: BackingView> PaneHeader<P> {
    pub fn set_shareable_object(
        &mut self,
        _shareable_object: Option<ShareableObject>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn sharing_dialog(&self) -> &ViewHandle<SharingDialogLite> {
        &self.shared_content.sharing_dialog
    }

    pub fn has_shareable_object<C: warpui::ViewAsRef>(&self, _ctx: &C) -> bool {
        false
    }

    pub fn has_shareable_shared_session<C: warpui::ViewAsRef>(&self, _ctx: &C) -> bool {
        false
    }

    pub fn is_sharing_dialog_enabled<C: warpui::ViewAsRef>(&self, _ctx: &C) -> bool {
        false
    }

    pub fn share_pane_contents(
        &mut self,
        _source: SharingDialogSource,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn open_shared_session_qr_code(
        &mut self,
        _source: SharingDialogSource,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    fn handle_sharing_dialog_event(
        &mut self,
        event: &SharingDialogEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        if matches!(event, SharingDialogEvent::Close) {
            self.close_overlay(ctx);
        }
    }

    pub fn render_sharing_controls(
        &self,
        _element: &mut impl ParentElement,
        _appearance: &Appearance,
        _icon_color_override: Option<warp_core::ui::theme::Fill>,
        _button_size_override: Option<f32>,
        _app: &AppContext,
    ) {
    }
}
