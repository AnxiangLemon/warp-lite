use std::sync::Arc;

use futures::executor::block_on;
use warp_graphql::client::GraphQLError;
use warp_server_auth::auth_state::AuthState;

use super::{AuthenticatedGraphqlConfig, BaseClient, GraphqlRoutingConfig};

struct UnreadableIapCredentials;

impl http_client::iap::IapTokenProvider for UnreadableIapCredentials {
    fn cached_token(&self) -> Option<String> {
        panic!("disabled transport must not read IAP credentials")
    }
}

#[test]
fn disabled_transport_never_reads_the_iap_token_provider() {
    let (sender, _) = async_channel::unbounded();
    let client = BaseClient::new(
        Arc::new(http_client::Client::new()),
        Arc::new(AuthState::new_for_test()),
        sender,
        None,
        GraphqlRoutingConfig::default(),
        AuthenticatedGraphqlConfig::default(),
        Some(Arc::new(UnreadableIapCredentials)),
    );

    assert_eq!(client.iap_proxy_auth_header(), None);
    assert_eq!(client.access_token_ignoring_validity(), None);
}

#[test]
fn disabled_transport_rejects_ambient_token_issuance() {
    let (sender, _) = async_channel::unbounded();
    let client = BaseClient::new(
        Arc::new(http_client::Client::new()),
        Arc::new(AuthState::new_for_test()),
        sender,
        None,
        GraphqlRoutingConfig::default(),
        AuthenticatedGraphqlConfig::default(),
        None,
    );

    let error = block_on(client.get_or_create_ambient_workload_token()).unwrap_err();

    assert!(matches!(
        error.downcast_ref::<GraphQLError>(),
        Some(GraphQLError::Unavailable)
    ));
}
