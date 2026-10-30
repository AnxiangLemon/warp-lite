use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use warp_graphql::managed_secrets::ManagedSecret;
use warpui_core::{Entity, SingletonEntity};

use crate::client::{IdentityTokenOptions, ManagedSecretsClient, SecretOwner, TaskIdentityToken};
use crate::{
    ActorProvider, GcpWorkloadIdentityFederationError, GcpWorkloadIdentityFederationToken,
    ManagedSecretValue,
};

/// Managed secret interface for builds without managed secret services.
pub struct ManagedSecretManager;

impl ManagedSecretManager {
    pub fn new(
        client: Arc<dyn ManagedSecretsClient>,
        actor_provider: Arc<dyn ActorProvider>,
    ) -> Self {
        let _ = (client, actor_provider);
        Self
    }

    pub fn create_secret(
        &self,
        owner: SecretOwner,
        name: String,
        value: ManagedSecretValue,
        description: Option<String>,
    ) -> impl Future<Output = anyhow::Result<ManagedSecret>> + use<> {
        let _ = (owner, name, value, description);
        async { Err(warp_graphql::client::GraphQLError::Unavailable.into()) }
    }

    pub fn delete_secret(
        &self,
        owner: SecretOwner,
        name: String,
    ) -> impl Future<Output = anyhow::Result<()>> + use<> {
        let _ = (owner, name);
        async { Err(warp_graphql::client::GraphQLError::Unavailable.into()) }
    }

    pub fn update_secret(
        &self,
        owner: SecretOwner,
        name: String,
        value: Option<ManagedSecretValue>,
        description: Option<String>,
    ) -> impl Future<Output = anyhow::Result<ManagedSecret>> + use<> {
        let _ = (owner, name, value, description);
        async { Err(warp_graphql::client::GraphQLError::Unavailable.into()) }
    }

    pub fn list_secrets(&self) -> impl Future<Output = anyhow::Result<Vec<ManagedSecret>>> + use<> {
        async { Err(warp_graphql::client::GraphQLError::Unavailable.into()) }
    }

    pub fn get_task_secrets(
        &self,
        task_id: String,
    ) -> impl Future<Output = anyhow::Result<HashMap<String, ManagedSecretValue>>> + use<> {
        let _ = task_id;
        async { Err(warp_graphql::client::GraphQLError::Unavailable.into()) }
    }

    pub fn issue_task_identity_token(
        &self,
        options: IdentityTokenOptions,
    ) -> impl Future<Output = anyhow::Result<TaskIdentityToken>> + use<> {
        let _ = options;
        async { Err(warp_graphql::client::GraphQLError::Unavailable.into()) }
    }

    pub fn issue_gcp_workload_identity_federation_token(
        &self,
        audience: String,
        token_type: String,
        requested_duration: Duration,
    ) -> impl Future<
        Output = Result<GcpWorkloadIdentityFederationToken, GcpWorkloadIdentityFederationError>,
    > + use<> {
        let _ = (audience, token_type, requested_duration);
        async {
            Err(GcpWorkloadIdentityFederationError::new(
                "Managed secret services are unavailable in this build",
            ))
        }
    }
}

impl Entity for ManagedSecretManager {
    type Event = ();
}

impl SingletonEntity for ManagedSecretManager {}
