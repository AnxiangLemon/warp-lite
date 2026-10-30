#[cfg(feature = "agent_runtime")]
use warpui::elements::ParentElement;
use warpui::presenter::ChildView;
use warpui::{AppContext, Element, FocusContext, SingletonEntity, View, ViewContext, ViewHandle};

use super::{NewWorkspaceSource, RootView, WorkspaceArgs, quake_mode_window_is_open};
use crate::GlobalResourceHandles;
use crate::server::server_api::ServerApiProvider;
use crate::settings_view::flags;
use crate::terminal::keys_settings::KeysSettings;
use crate::workspace::Workspace;
use crate::workspaces::user_workspaces::UserWorkspaces;

impl RootView {
    pub fn new(
        global_resource_handles: GlobalResourceHandles,
        workspace_setting: NewWorkspaceSource,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let window_id = ctx.window_id();
        UserWorkspaces::handle(ctx).update(ctx, |workspaces, ctx| {
            workspaces.register_window(window_id, None, ctx);
        });
        let model_event_sender = global_resource_handles.model_event_sender.clone();
        let server_api = ServerApiProvider::as_ref(ctx).get().clone();
        let workspace = WorkspaceArgs {
            global_resource_handles,
            server_time: None,
            workspace_setting,
        }
        .create_workspace(ctx);
        Self {
            workspace,
            server_api,
            model_event_sender,
        }
    }

    pub fn workspace_view(&self) -> Option<&ViewHandle<Workspace>> {
        Some(&self.workspace)
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) -> bool {
        ctx.focus(&self.workspace);
        ctx.notify();
        true
    }
}

impl View for RootView {
    fn ui_name() -> &'static str {
        "RootView"
    }

    fn on_focus(&mut self, focus_ctx: &FocusContext, ctx: &mut ViewContext<Self>) {
        if focus_ctx.is_self_focused() {
            self.focus(ctx);
        }
    }

    fn render(&self, _: &AppContext) -> Box<dyn Element> {
        ChildView::new(&self.workspace).finish()
    }

    fn keymap_context(&self, app: &AppContext) -> warpui::keymap::Context {
        use settings::Setting as _;

        let mut context = Self::default_keymap_context();
        if quake_mode_window_is_open() {
            context.set.insert(flags::QUAKE_WINDOW_OPEN_FLAG);
        }
        if *KeysSettings::as_ref(app).quake_mode_enabled {
            context.set.insert(flags::QUAKE_MODE_ENABLED_CONTEXT_FLAG);
        }
        if *KeysSettings::as_ref(app).activation_hotkey_enabled.value() {
            context.set.insert(flags::ACTIVATION_HOTKEY_FLAG);
        }
        context
    }
}
