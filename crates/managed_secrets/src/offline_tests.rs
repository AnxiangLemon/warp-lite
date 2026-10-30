use std::sync::Arc;
use std::time::Duration;

use warp_graphql::client::GraphQLError;
use warpui_core::r#async::block_on;

use crate::client::ManagedSecretsClient;
use crate::{
    ActorProvider, GcpCredentials, GcpFederationConfig, ManagedSecretManager,
    PrepareGcpCredentialsError, UploadKey,
};

struct OfflineServices;

impl ManagedSecretsClient for OfflineServices {}

impl ActorProvider for OfflineServices {
    fn actor_uid(&self) -> Option<String> {
        None
    }
}

#[test]
fn disabled_secret_listing_returns_unavailable_instead_of_an_empty_list() {
    let manager = ManagedSecretManager::new(Arc::new(OfflineServices), Arc::new(OfflineServices));

    let error = block_on(manager.list_secrets()).unwrap_err();

    assert!(matches!(
        error.downcast_ref::<GraphQLError>(),
        Some(GraphQLError::Unavailable)
    ));
}

#[test]
fn disabled_gcp_token_issuance_returns_a_failure_payload() {
    let manager = ManagedSecretManager::new(Arc::new(OfflineServices), Arc::new(OfflineServices));

    let error = block_on(manager.issue_gcp_workload_identity_federation_token(
        "test-audience".into(),
        "urn:ietf:params:oauth:token-type:id_token".into(),
        Duration::from_secs(60),
    ))
    .unwrap_err();

    assert!(!error.success);
    assert_eq!(error.code, "TOKEN_ISSUANCE_FAILED");
    assert_eq!(
        error.message,
        "Managed secret services are unavailable in this build"
    );
}

#[test]
fn disabled_encryption_rejects_key_import_before_parsing() {
    let error = UploadKey::import_public_keyset("not-a-key")
        .err()
        .expect("disabled encryption must not return a key handle");

    assert!(matches!(error, crate::envelope::EnvelopeError::Unavailable));
}

#[test]
fn disabled_gcp_credentials_reject_config_file_creation() {
    let config = GcpFederationConfig {
        project_number: "test-project".into(),
        pool_id: "test-pool".into(),
        provider_id: "test-provider".into(),
        service_account_email: None,
        token_lifetime: None,
    };

    let error = GcpCredentials::federated("test-task", &config)
        .err()
        .expect("disabled federation must not create a credential handle");

    assert!(matches!(error, PrepareGcpCredentialsError::Unavailable));
}
