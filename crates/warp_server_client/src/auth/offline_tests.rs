use std::sync::Arc;

use futures::executor::block_on;
use warp_graphql::client::GraphQLError;
use warp_server_auth::auth_state::AuthState;
use warp_server_auth::credentials::LoginToken;

use super::{AuthClient, AuthSession, UserAuthenticationError};

struct OfflineAuthClient;

impl AuthClient for OfflineAuthClient {}

#[test]
fn account_lookup_defaults_report_unavailable() {
    let error = block_on(OfflineAuthClient.list_api_keys()).unwrap_err();

    assert!(matches!(
        error.downcast_ref::<GraphQLError>(),
        Some(GraphQLError::Unavailable)
    ));
}

#[test]
fn offline_session_does_not_accept_api_key_credentials() {
    let (sender, receiver) = async_channel::unbounded();
    let session = AuthSession::new(
        Arc::new(http_client::Client::new()),
        Arc::new(AuthState::new_logged_out_for_test()),
        sender,
    );

    let result = block_on(session.exchange_credentials(LoginToken::ApiKey("test-key".into())));

    assert!(matches!(
        result,
        Err(UserAuthenticationError::Unexpected(_))
    ));
    assert!(!session.allowed_to_refresh_token());
    assert!(receiver.try_recv().is_err());
}
