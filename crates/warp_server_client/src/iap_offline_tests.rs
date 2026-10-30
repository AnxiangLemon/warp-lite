use std::sync::Arc;

use warp_core::channel::IapConfig;
use warpui_core::App;

use super::{IapCredentialsState, IapManager, IapState};

#[test]
fn disabled_iap_state_never_exposes_cached_credentials() {
    let state = IapState::new(&IapConfig {
        audiences: "test-audience".into(),
        service_account_email: "test-account@example.invalid".into(),
    });

    assert!(matches!(
        state.state(),
        IapCredentialsState::Failed { message, previous: None }
            if message == "IAP authentication is unavailable in this build"
    ));
    assert_eq!(state.get_cached(), None);
    assert_eq!(state.proxy_auth_header(), None);
}

#[test]
fn disabled_iap_manager_never_starts_the_shell_path_helper() {
    App::test((), |app| async move {
        let state = Arc::new(IapState::new(&IapConfig {
            audiences: "test-audience".into(),
            service_account_email: "test-account@example.invalid".into(),
        }));
        let model = app.add_singleton_model(|ctx| {
            IapManager::new(
                Some(state),
                Box::new(|_| panic!("disabled IAP must not launch the shell PATH helper")),
                None,
                ctx,
            )
        });

        model.read(&app, |manager, _| {
            assert!(!manager.is_enabled());
            assert!(!manager.has_valid_token());
            assert!(manager.iap_state().is_none());
        });
    });
}
