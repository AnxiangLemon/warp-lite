use std::sync::Arc;

use warp_core::channel::ChannelState;
use warp_server_auth::user::User;
use warpui::{Entity, ModelContext, SingletonEntity};

use super::auth_state::AuthState;
use super::credentials::Credentials;
use super::{AuthStateProvider, UserUid};
use crate::server::server_api::auth::{AuthClient, UserAuthenticationError};
use crate::server::server_api::ServerApi;
use crate::server::telemetry::AnonymousUserSignupEntrypoint;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthViewVariant {
    Initial,
    RequireLoginCloseable,
    HitDriveObjectLimitCloseable,
    ShareRequirementCloseable,
}

#[derive(Clone, Debug)]
pub struct AuthRedirectPayload {
    pub refresh_token: String,
    pub user_uid: Option<UserUid>,
    pub deleted_anonymous_user: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug)]
pub enum AuthManagerEvent {
    AuthComplete,
    AuthFailed(UserAuthenticationError),
    CreateAnonymousUserFailed,
    SkippedLogin,
    NeedsReauth,
    AttemptedLoginGatedFeature {
        auth_view_variant: AuthViewVariant,
    },
    LoginOverrideDetected(AuthRedirectPayload),
    MintCustomTokenFailed(
        crate::server::server_api::auth::MintCustomTokenError,
    ),
    ReceivedDeviceAuthorizationCode {
        verification_url: String,
        verification_url_complete: Option<String>,
        user_code: String,
    },
}

pub type LoginGatedFeature = &'static str;

pub struct AuthManager {
    auth_state: Arc<AuthState>,
}

impl AuthManager {
    pub fn new(
        _server_api: Arc<ServerApi>,
        _auth_client: Arc<dyn AuthClient>,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            auth_state: AuthStateProvider::as_ref(ctx).get().clone(),
        }
    }

    #[cfg(any(test, all(feature = "tui", feature = "test-util")))]
    pub fn new_for_test(ctx: &mut ModelContext<Self>) -> Self {
        Self {
            auth_state: AuthStateProvider::as_ref(ctx).get().clone(),
        }
    }

    pub fn initialize_user_from_auth_payload(
        &mut self,
        _auth_payload: AuthRedirectPayload,
        _enforce_state_validation: bool,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn resume_interrupted_auth_payload(
        &mut self,
        _auth_payload: AuthRedirectPayload,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn initialize_user_from_session_cookie(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn refresh_user(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn authenticate_api_key(&self, _api_key: String, _ctx: &mut ModelContext<Self>) {}

    pub fn authorize_device(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn set_needs_reauth(&self, needs_reauth: bool, _ctx: &mut ModelContext<Self>) {
        self.auth_state.set_needs_reauth(needs_reauth);
    }

    pub fn create_anonymous_user(
        &self,
        _referral_code: Option<String>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn attempt_login_gated_feature(
        &self,
        _feature: LoginGatedFeature,
        _auth_view_variant: AuthViewVariant,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn anonymous_user_hit_drive_object_limit(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn initiate_anonymous_user_linking(
        &self,
        _entrypoint: AnonymousUserSignupEntrypoint,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn open_url_maybe_with_anonymous_token(
        &self,
        _url: &str,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn copy_anonymous_user_linking_url_to_clipboard(&self, _ctx: &mut ModelContext<Self>) {}

    pub fn sign_up_url(&mut self) -> String {
        format!("{}/signup", ChannelState::server_root_url())
    }

    pub fn sign_in_url(&mut self) -> String {
        format!("{}/login", ChannelState::server_root_url())
    }

    pub fn upgrade_url(&mut self) -> String {
        format!("{}/upgrade", ChannelState::server_root_url())
    }

    pub fn login_options_url(&mut self, _custom_token: &str) -> String {
        format!("{}/login", ChannelState::server_root_url())
    }

    pub fn link_sso_url(&mut self, email: &str) -> String {
        format!("{}/link_sso?email={email}", ChannelState::server_root_url())
    }

    pub fn set_user_onboarded(&self, _ctx: &mut ModelContext<Self>) {
        self.auth_state.set_is_onboarded(true);
    }

    pub(super) fn log_out(&mut self, _ctx: &mut ModelContext<Self>) {
        self.auth_state.set_user(Some(User::test()));
        self.auth_state.set_credentials(Some(Credentials::Test));
        self.auth_state.set_needs_reauth(false);
    }
}

#[derive(Clone, Debug)]
pub struct PersistedCurrentUserInformation {
    pub email: String,
}

impl Entity for AuthManager {
    type Event = AuthManagerEvent;
}

impl SingletonEntity for AuthManager {}
