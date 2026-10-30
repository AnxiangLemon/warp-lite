use warpui::integration::TestStep;
use warpui::{SingletonEntity, async_assert, async_assert_eq};

use crate::auth::AuthStateProvider;
#[cfg(feature = "agent_runtime")]
use crate::cloud_object::model::persistence::CloudModel;
#[cfg(feature = "agent_runtime")]
use crate::cloud_object::{CloudObjectEventEntrypoint, CloudObjectLocation, Space};
use crate::network::{NetworkStatus, NetworkStatusKind};
#[cfg(feature = "agent_runtime")]
use crate::server::cloud_objects::listener::Listener;
#[cfg(feature = "agent_runtime")]
use crate::server::cloud_objects::update_manager::UpdateManager;
#[cfg(feature = "agent_runtime")]
use crate::server::ids::ClientId;
use crate::util::bindings::keybinding_name_to_display_string;
#[cfg(feature = "agent_runtime")]
use crate::workflows::workflow::Workflow;
#[cfg(feature = "agent_runtime")]
use crate::workspaces::team::Team;
#[cfg(feature = "agent_runtime")]
use crate::workspaces::user_workspaces::UserWorkspaces;
#[cfg(feature = "agent_runtime")]
use crate::workspaces::workspace::Workspace;

pub fn assert_logged_out() -> TestStep {
    TestStep::new("Assert no account is active").add_assertion(|app, _| {
        AuthStateProvider::handle(app).read(app, |provider, _| {
            async_assert!(
                !provider.get().is_logged_in(),
                "No account should be active"
            )
        })
    })
}

pub fn assert_lite_runtime_services_are_absent() -> TestStep {
    TestStep::new("Assert Lite has no AI or Git runtime models or hidden login views").add_assertion(
        |app, window_id| {
            crate::integration_testing::view_getters::workspace_view(app, window_id).read(
                app,
                |_, ctx| {
                    let registered = [
                        ("local workflows", ctx.has_singleton_model::<crate::workflows::local_workflows::LocalWorkflows>()),
                        ("Git status", ctx.has_singleton_model::<crate::code_review::git_repo_model::GitRepoModels>()),
                        #[cfg(feature = "local_fs")]
                        ("Git repository scan", ctx.has_singleton_model::<repo_metadata::RepoMetadataModel>()),
                        ("AI usage", ctx.has_singleton_model::<crate::ai::request_usage_model::AIRequestUsageModel>()),
                        ("provider keys", ctx.has_singleton_model::<::ai::api_keys::ApiKeyManager>()),
                        ("agent history", ctx.has_singleton_model::<crate::ai::blocklist::BlocklistAIHistoryModel>()),
                        ("active agents", ctx.has_singleton_model::<crate::ai::active_agent_views_model::ActiveAgentViewsModel>()),
                        ("CLI agents", ctx.has_singleton_model::<crate::terminal::cli_agent_sessions::CLIAgentSessionsModel>()),
                        ("MCP", ctx.has_singleton_model::<crate::ai::mcp::TemplatableMCPServerManager>()),
                        ("code index", ctx.has_singleton_model::<::ai::index::full_source_code_embedding::manager::CodebaseIndexManager>()),
                        ("project context", ctx.has_singleton_model::<::ai::project_context::model::ProjectContextModel>()),
                    ];
                    let login_views = [
                        ("authentication", ctx.views_of_type::<crate::auth::auth_view_modal::AuthView>(window_id).expect("workspace window exists").len()),
                        ("authentication override", ctx.views_of_type::<crate::auth::auth_override_warning_modal::AuthOverrideWarningModal>(window_id).expect("workspace window exists").len()),
                        ("SSO", ctx.views_of_type::<crate::auth::needs_sso_link_view::NeedsSsoLinkView>(window_id).expect("workspace window exists").len()),
                        ("login slide", ctx.views_of_type::<crate::auth::login_slide::LoginSlideView>(window_id).expect("workspace window exists").len()),
                        ("agent onboarding", ctx.views_of_type::<onboarding::AgentOnboardingView>(window_id).expect("workspace window exists").len()),
                        ("paste authentication token", ctx.views_of_type::<crate::auth::paste_auth_token_modal::PasteAuthTokenModalView>(window_id).expect("workspace window exists").len()),
                    ];
                    async_assert!(
                        registered.iter().all(|(_, present)| !present)
                            && login_views.iter().all(|(_, count)| *count == 0),
                        "Unexpected runtime models: {registered:?}; hidden login views: {login_views:?}"
                    )
                },
            )
        },
    )
}

fn set_and_assert_network_status(status: NetworkStatusKind) -> TestStep {
    TestStep::new("Set and assert network status")
        .with_action(move |app, _, _| {
            NetworkStatus::handle(app).update(app, |network_status, ctx| {
                if matches!(status, NetworkStatusKind::Online) {
                    network_status.reachability_changed(true, ctx);
                } else {
                    network_status.reachability_changed(false, ctx);
                }
            });
        })
        .add_assertion(move |app, _| {
            NetworkStatus::handle(app).read(app, |network_status, _| {
                async_assert!(
                    network_status.status() == status,
                    "network status is correct"
                )
            })
        })
}

pub fn go_offline() -> TestStep {
    set_and_assert_network_status(NetworkStatusKind::Offline)
}

pub fn go_online() -> TestStep {
    set_and_assert_network_status(NetworkStatusKind::Online)
}

#[cfg(feature = "agent_runtime")]
pub fn join_a_workspace() -> TestStep {
    TestStep::new("Join a Warp Drive workspace")
        .with_action(move |app, _, _| {
            UserWorkspaces::handle(app).update(app, |user_workspaces, ctx| {
                let workspace_uid = "workspace_uid123456789".to_string().into();
                let teams: Vec<Team> = vec![Team {
                    uid: "team_uid12345678912345".try_into().expect("ID is valid"),
                    name: "My Team".to_string(),
                    color: None,
                    invite_code: Default::default(),
                    members: Default::default(),
                    pending_email_invites: Default::default(),
                    invite_link_domain_restrictions: Default::default(),
                    billing_metadata: Default::default(),
                    stripe_customer_id: None,
                    settings: Default::default(),
                    is_eligible_for_discovery: false,
                    has_billing_history: false,
                }];
                let workspaces: Vec<Workspace> = vec![Workspace {
                    uid: workspace_uid,
                    name: "My Workspace".to_string(),
                    stripe_customer_id: None,
                    teams: teams.clone(),
                    billing_metadata: Default::default(),
                    bonus_grants_purchased_this_month: Default::default(),
                    billing_cycle_usage: None,
                    has_billing_history: false,
                    settings: Default::default(),
                    invite_code: Default::default(),
                    invite_link_domain_restrictions: Default::default(),
                    pending_email_invites: Default::default(),
                    is_eligible_for_discovery: false,
                    members: Default::default(),
                    total_requests_used_since_last_refresh: 0,
                }];

                user_workspaces.update_workspaces(workspaces, ctx);
                user_workspaces.set_current_workspace_uid(workspace_uid, ctx)
            });
        })
        .add_assertion(move |app, _| {
            UserWorkspaces::handle(app).read(app, |user_workspaces, _| {
                async_assert!(user_workspaces.has_teams(), "user is on a team")
            })
        })
        .add_assertion(move |app, _| {
            UserWorkspaces::handle(app).read(app, |user_workspaces, _| {
                async_assert!(user_workspaces.has_workspaces(), "user is on a workspace")
            })
        })
}

#[cfg(feature = "agent_runtime")]
pub fn create_a_personal_workflow() -> TestStep {
    TestStep::new("Create a personal workflow")
        .with_action(move |app, _, _| {
            UpdateManager::handle(app).update(app, |update_manager, ctx| {
                update_manager.create_workflow(
                    Workflow::new("My first workflow", "ls"),
                    UserWorkspaces::as_ref(ctx)
                        .personal_drive(ctx)
                        .expect("User UID must be set in tests"),
                    None,
                    ClientId::default(),
                    CloudObjectEventEntrypoint::ManagementUI,
                    true,
                    ctx,
                )
            })
        })
        .add_assertion(move |app, _| {
            CloudModel::handle(app).read(app, |cloud_model, ctx| {
                async_assert!(
                    cloud_model
                        .active_cloud_objects_in_location_without_descendents(
                            CloudObjectLocation::Space(Space::Personal),
                            ctx,
                        )
                        .count()
                        > 0,
                    "cloud objects exist"
                )
            })
        })
}

pub fn assert_binding_display_string(
    binding: &'static str,
    display_string: Option<&'static str>,
) -> TestStep {
    TestStep::new("Assert a binding's display string").add_named_assertion(
        format!("Binding {binding} should have display string {display_string:?}"),
        move |app, _| {
            app.update(|ctx| {
                async_assert_eq!(
                    keybinding_name_to_display_string(binding, ctx).as_deref(),
                    display_string
                )
            })
        },
    )
}

#[cfg(feature = "agent_runtime")]
pub fn assert_websocket_has_started() -> TestStep {
    TestStep::new("Assert a websocket has started").add_named_assertion(
        "subscription abort handle should exist",
        move |app, _| {
            Listener::handle(app).read(app, |listener, _| {
                async_assert!(
                    listener.has_current_subscription_abort_handle(),
                    "subscription has started"
                )
            })
        },
    )
}

#[cfg(feature = "agent_runtime")]
pub fn assert_websocket_has_not_started() -> TestStep {
    TestStep::new("Assert a websocket has not started").add_named_assertion(
        "subscription abort handle should not exist",
        move |app, _| {
            Listener::handle(app).read(app, |listener, _| {
                async_assert!(
                    !listener.has_current_subscription_abort_handle(),
                    "subscription has not started"
                )
            })
        },
    )
}
