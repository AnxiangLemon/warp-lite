use warpui::platform::WindowStyle;
use warpui::{App, ViewHandle};

use super::{NewWorkspaceSource, RootView};
use crate::GlobalResourceHandles;
use crate::auth::auth_override_warning_modal::AuthOverrideWarningModal;
use crate::auth::auth_view_modal::AuthView;
use crate::auth::login_slide::LoginSlideView;
use crate::auth::needs_sso_link_view::NeedsSsoLinkView;
use crate::auth::paste_auth_token_modal::PasteAuthTokenModalView;
use crate::workspace::Workspace;

#[test]
#[cfg(feature = "warp_lite")]
fn lite_root_starts_a_real_workspace_without_hidden_login_or_onboarding_views() {
    App::test((), |mut app| async move {
        crate::workspace::view::tests::initialize_app(&mut app);
        app.update(super::init);
        let resources = GlobalResourceHandles::mock(&mut app);
        let (window_id, root) = app.add_window(WindowStyle::NotStealFocus, |ctx| {
            RootView::new(
                resources,
                NewWorkspaceSource::Empty {
                    previous_active_window: None,
                    shell: None,
                },
                ctx,
            )
        });
        let workspace: ViewHandle<Workspace> = root.read(&app, |root, _| {
            root.workspace_view()
                .expect("Native root has a workspace")
                .clone()
        });
        workspace.read(&app, |workspace, ctx| {
            assert_eq!(workspace.tab_count(), 1);
            assert!(
                workspace
                    .active_tab_pane_group()
                    .as_ref(ctx)
                    .active_session_view(ctx)
                    .is_some()
            );
        });
        app.read(|ctx| {
            assert!(ctx.views_of_type::<AuthView>(window_id).unwrap().is_empty());
            assert!(
                ctx.views_of_type::<AuthOverrideWarningModal>(window_id)
                    .unwrap()
                    .is_empty()
            );
            assert!(
                ctx.views_of_type::<NeedsSsoLinkView>(window_id)
                    .unwrap()
                    .is_empty()
            );
            assert!(
                ctx.views_of_type::<LoginSlideView>(window_id)
                    .unwrap()
                    .is_empty()
            );
            assert!(
                ctx.views_of_type::<onboarding::AgentOnboardingView>(window_id)
                    .unwrap()
                    .is_empty()
            );
            assert!(
                ctx.views_of_type::<PasteAuthTokenModalView>(window_id)
                    .unwrap()
                    .is_empty()
            );
            assert!(
                !ctx.editable_bindings()
                    .any(|binding| binding.name == "root_view:enter_onboarding_state")
            );
            assert!(
                ctx.editable_bindings()
                    .any(|binding| binding.name == "root_view:toggle_fullscreen")
            );
        });
    });
}
