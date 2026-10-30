use std::collections::HashMap;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::credential_storage::{
    load_credentials_from_secure_storage, write_to_secure_storage,
};

pub const TEMPLATABLE_MCP_CREDENTIALS_KEY: &str = "TemplatableMcpCredentials";
pub const FILE_BASED_MCP_CREDENTIALS_KEY: &str = "FileBasedMcpCredentials";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedCredentials {
    #[serde(flatten)]
    credentials: serde_json::Map<String, serde_json::Value>,
    client_secret: Option<String>,
}

pub type PersistedCredentialsMap = HashMap<Uuid, PersistedCredentials>;
pub type FileBasedPersistedCredentialsMap = HashMap<u64, PersistedCredentials>;
pub type PersistCredentialsCallback =
    Box<dyn Fn(Uuid, PersistedCredentials) -> BoxFuture<'static, anyhow::Result<()>> + Send>;
pub type RequiresAuthenticationCallback =
    Box<dyn Fn(Uuid, String, String) -> BoxFuture<'static, anyhow::Result<()>> + Send>;
pub type AuthenticatedCallback =
    Box<dyn Fn(String) -> BoxFuture<'static, anyhow::Result<()>> + Send>;

pub enum OAuthCallbackMode {
    CustomScheme {
        redirect_uri: String,
        result_rx: async_channel::Receiver<CallbackResult>,
    },
    Loopback,
}

pub struct AuthContext {
    pub callback_mode: OAuthCallbackMode,
    pub uuid: Uuid,
    pub persisted_credentials: Option<PersistedCredentials>,
    pub is_headless: bool,
    pub is_file_based: bool,
    pub persist_credentials: PersistCredentialsCallback,
    pub requires_authentication: RequiresAuthenticationCallback,
    pub authenticated: Option<AuthenticatedCallback>,
}

#[derive(Debug, Clone)]
pub enum CallbackResult {
    Success { code: String, csrf_token: String },
    Error { error: Option<String> },
}

#[cfg(test)]
#[path = "oauth_disabled_tests.rs"]
mod tests;
