pub mod client;
#[cfg(feature = "runtime")]
mod envelope;
#[cfg(not(feature = "runtime"))]
#[path = "envelope_offline.rs"]
mod envelope;
mod gcp;
#[cfg(feature = "runtime")]
mod manager;
#[cfg(not(feature = "runtime"))]
#[path = "manager_offline.rs"]
mod manager;
mod secret_value;

pub use client::TaskIdentityToken;
pub use envelope::UploadKey;
#[cfg(feature = "runtime")]
pub use envelope::init as init_envelope;
pub use gcp::{
    GcpCredentials, GcpFederationConfig, GcpWorkloadIdentityFederationError,
    GcpWorkloadIdentityFederationToken, PrepareGcpCredentialsError,
};
pub use manager::ManagedSecretManager;
pub use secret_value::ManagedSecretValue;

pub trait ActorProvider: Send + Sync + 'static {
    fn actor_uid(&self) -> Option<String>;
}

#[cfg(all(test, not(feature = "runtime")))]
#[path = "offline_tests.rs"]
mod tests;
