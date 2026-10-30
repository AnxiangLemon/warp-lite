use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::Result;
use instant::Instant;
use warp_core::channel::IapConfig;
use warpui_core::AppContext;
use warpui_core::r#async::BoxFuture;

#[cfg(feature = "network_transport")]
#[path = "iap_runtime.rs"]
mod runtime;
#[cfg(not(feature = "network_transport"))]
#[path = "iap_offline.rs"]
mod runtime;

pub use runtime::IapManager;
#[cfg(any(not(target_family = "wasm"), not(feature = "network_transport")))]
pub use runtime::ws_connect_is_iap_challenge;

pub type PathResolver = Box<dyn Fn(&mut AppContext) -> BoxFuture<'static, Option<String>>>;

/// Mints a Warp-signed OIDC identity token for a given audience. Implemented in
/// the `app` crate over the managed-secrets client so this crate need not depend
/// on the managed-secrets stack.
pub trait IapIdentityTokenMinter: Send + Sync + 'static {
    fn mint_identity_token(
        &self,
        audience: String,
        requested_duration: Duration,
    ) -> BoxFuture<'static, Result<String>>;
}

/// Lets a sandboxed Oz runner self-mint IAP tokens via Workload Identity
/// Federation. Present only in runner context.
#[derive(Clone)]
pub struct ManagedIapMint {
    #[cfg(feature = "network_transport")]
    minter: Arc<dyn IapIdentityTokenMinter>,
}

impl ManagedIapMint {
    pub fn new(minter: Arc<dyn IapIdentityTokenMinter>) -> Self {
        #[cfg(not(feature = "network_transport"))]
        drop(minter);
        Self {
            #[cfg(feature = "network_transport")]
            minter,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CachedToken {
    pub token: String,
    pub expires_at: Instant,
}

impl CachedToken {
    #[cfg(feature = "network_transport")]
    fn valid_token(&self) -> Option<String> {
        (self.expires_at > Instant::now()).then(|| self.token.clone())
    }
}

#[derive(Debug, Clone)]
pub enum IapCredentialsState {
    Missing,
    /// A credential fetch is in progress. `previous` carries the last
    /// successfully-loaded token (if any). Allows us to attach it to
    /// outbound requests while we're refreshing so that proactive refreshes
    /// (i.e. refresh the token 5min before exp) don't prevent active requests.
    Refreshing {
        previous: Option<CachedToken>,
    },
    Loaded(CachedToken),
    Failed {
        message: String,
        // in case the last token still works... we can try to use that for a couple more mins
        previous: Option<CachedToken>,
    },
}

impl IapCredentialsState {
    #[cfg(feature = "network_transport")]
    fn previous_token(&self) -> Option<CachedToken> {
        match self {
            IapCredentialsState::Loaded(cached) => Some(cached.clone()),
            IapCredentialsState::Refreshing { previous }
            | IapCredentialsState::Failed { previous, .. } => previous.clone(),
            IapCredentialsState::Missing => None,
        }
    }
}

pub struct IapState {
    audiences: String,
    service_account_email: String,
    inner: RwLock<IapCredentialsState>,
}

impl IapState {
    pub fn new(config: &IapConfig) -> Self {
        Self {
            audiences: config.audiences.to_string(),
            service_account_email: config.service_account_email.to_string(),
            inner: RwLock::new(if cfg!(feature = "network_transport") {
                IapCredentialsState::Missing
            } else {
                IapCredentialsState::Failed {
                    message: "IAP authentication is unavailable in this build".into(),
                    previous: None,
                }
            }),
        }
    }

    pub fn get_cached(&self) -> Option<String> {
        #[cfg(not(feature = "network_transport"))]
        {
            None
        }
        #[cfg(feature = "network_transport")]
        {
            match &*self.inner.read().expect("IAP state lock poisoned") {
                // Gate on expiry even while `Loaded`: if a proactive refresh is
                // delayed (e.g. the machine slept across the refresh window), the
                // token may already be expired, and attaching it would guarantee an
                // IAP challenge. Returning `None` lets the caller proceed without a
                // doomed token while the reactive refresh recovers.
                IapCredentialsState::Loaded(cached) => cached.valid_token(),
                IapCredentialsState::Refreshing { previous }
                | IapCredentialsState::Failed { previous, .. } => {
                    previous.as_ref().and_then(CachedToken::valid_token)
                }
                IapCredentialsState::Missing => None,
            }
        }
    }

    pub fn proxy_auth_header(&self) -> Option<(&'static str, String)> {
        self.get_cached()
            .map(|token| http_client::iap::proxy_auth_header(&token))
    }

    pub fn state(&self) -> IapCredentialsState {
        self.inner.read().expect("IAP state lock poisoned").clone()
    }

    pub fn audiences(&self) -> &str {
        &self.audiences
    }

    pub fn service_account_email(&self) -> &str {
        &self.service_account_email
    }

    #[cfg(feature = "network_transport")]
    fn set_refreshing(&self) {
        let mut state = self.inner.write().expect("IAP state lock poisoned");
        *state = IapCredentialsState::Refreshing {
            previous: state.previous_token(),
        };
    }

    #[cfg(feature = "network_transport")]
    fn set_loaded(&self, cached: CachedToken) {
        *self.inner.write().expect("IAP state lock poisoned") = IapCredentialsState::Loaded(cached);
    }

    #[cfg(feature = "network_transport")]
    fn set_failed(&self, message: String) {
        let mut state = self.inner.write().expect("IAP state lock poisoned");
        *state = IapCredentialsState::Failed {
            message,
            previous: state.previous_token(),
        };
    }
}

impl http_client::iap::IapTokenProvider for IapState {
    fn cached_token(&self) -> Option<String> {
        self.get_cached()
    }
}

pub enum IapManagerEvent {
    StateChanged,
    /// Emitted when IAP access could not be established within the gate timeout,
    /// so a blocking caller can fail closed instead of hanging.
    AccessUnavailable,
    RefreshFailed {
        /// A human-readable error message describing why the refresh failed.
        message: String,
        /// Whether this is the first failure in a streak of failures.
        is_first_failure_of_streak: bool,
    },
}

#[cfg(all(test, not(feature = "network_transport")))]
#[path = "iap_offline_tests.rs"]
mod tests;
