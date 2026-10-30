use std::sync::Arc;

use anyhow::Result;
use instant::Duration;
use warp_server_auth::auth_state::AuthState;
use warp_server_auth::credentials::{AuthToken, Credentials, FirebaseToken, LoginToken};

use super::{AuthEvent, UserAuthenticationError, unavailable_authentication};

/// Authentication interface for builds without cloud transport.
pub struct AuthSession;

impl AuthSession {
    pub fn new(
        client: Arc<http_client::Client>,
        auth_state: Arc<AuthState>,
        event_sender: async_channel::Sender<AuthEvent>,
    ) -> Self {
        let _ = (client, auth_state, event_sender);
        Self
    }

    pub fn allowed_to_refresh_token(&self) -> bool {
        false
    }

    pub async fn get_or_refresh_access_token(&self) -> Result<AuthToken> {
        Err(unavailable_authentication())
    }

    pub async fn exchange_credentials(
        &self,
        token: LoginToken,
    ) -> Result<Credentials, UserAuthenticationError> {
        let _ = token;
        Err(UserAuthenticationError::Unexpected(
            unavailable_authentication(),
        ))
    }

    pub async fn request_device_code(
        &self,
    ) -> Result<oauth2::StandardDeviceAuthorizationResponse, UserAuthenticationError> {
        Err(UserAuthenticationError::Unexpected(
            unavailable_authentication(),
        ))
    }

    pub async fn exchange_device_access_token(
        &self,
        details: &oauth2::StandardDeviceAuthorizationResponse,
        timeout: Duration,
    ) -> Result<FirebaseToken, UserAuthenticationError> {
        let _ = (details, timeout);
        Err(UserAuthenticationError::Unexpected(
            unavailable_authentication(),
        ))
    }
}
