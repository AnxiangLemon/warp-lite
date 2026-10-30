use std::sync::Arc;

use warpui_core::{Entity, ModelContext, SingletonEntity};

use super::{IapCredentialsState, IapManagerEvent, IapState, ManagedIapMint, PathResolver};

/// Disabled IAP manager for builds without cloud transport.
pub struct IapManager {
    state: Option<Arc<IapState>>,
}

impl IapManager {
    pub fn new(
        state: Option<Arc<IapState>>,
        path_resolver: PathResolver,
        managed_mint: Option<ManagedIapMint>,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        let _ = (path_resolver, managed_mint, ctx);
        Self { state }
    }

    pub fn is_enabled(&self) -> bool {
        false
    }

    pub fn state(&self) -> Option<IapCredentialsState> {
        self.state.as_ref().map(|state| state.state())
    }

    pub fn iap_state(&self) -> Option<Arc<IapState>> {
        None
    }

    pub fn has_valid_token(&self) -> bool {
        false
    }

    pub fn ensure_access(&mut self, ctx: &mut ModelContext<Self>) {
        ctx.emit(IapManagerEvent::AccessUnavailable);
    }

    pub fn handle_challenge(&mut self, ctx: &mut ModelContext<Self>) {
        ctx.emit(IapManagerEvent::AccessUnavailable);
    }

    pub fn start_refresh(&mut self, ctx: &mut ModelContext<Self>) {
        ctx.emit(IapManagerEvent::AccessUnavailable);
    }

    pub fn check_ws_connect_error(&mut self, error: &anyhow::Error, ctx: &mut ModelContext<Self>) {
        let _ = (error, ctx);
    }
}

pub fn ws_connect_is_iap_challenge(error: &anyhow::Error) -> bool {
    let _ = error;
    false
}

impl Entity for IapManager {
    type Event = IapManagerEvent;
}

impl SingletonEntity for IapManager {}
