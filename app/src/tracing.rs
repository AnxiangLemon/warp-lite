#[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
use std::time::Duration;

use tracing::subscriber;

#[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
mod cloud_agent_auth;
#[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
mod native;

#[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
const DEFAULT_EXPORT_TIMEOUT: Duration = Duration::from_secs(10);

pub fn init() -> anyhow::Result<Initialization> {
    #[cfg(any(not(feature = "cloud_services"), target_family = "wasm"))]
    {
        install_no_subscriber()?;
        Ok(Initialization::default())
    }

    #[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
    native::init()
}

#[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
/// Starts cloud-agent trace credential refresh after authenticated application services exist.
///
/// The exporter and dispatch credential are initialized earlier by [`init`]. This later lifecycle
/// hook supplies the authenticated managed-secrets client needed to mint replacements without
/// broadening tracing initialization to ordinary application processes.
pub fn start_auth_refresh(
    client: std::sync::Arc<dyn warp_managed_secrets::client::ManagedSecretsClient>,
    ctx: &mut warpui::AppContext,
) {
    native::start_auth_refresh(client, ctx);
}

#[cfg(all(not(feature = "cloud_services"), not(target_family = "wasm")))]
pub fn start_auth_refresh(
    client: std::sync::Arc<dyn warp_managed_secrets::client::ManagedSecretsClient>,
    ctx: &mut warpui::AppContext,
) {
    let _ = (client, ctx);
}

fn install_no_subscriber() -> anyhow::Result<()> {
    // Configure the global tracing subscriber to not care about any spans or
    // events.
    //
    // This is done so that we prevent the `tracing` crate from writing out log
    // lines for spans and trace events.
    subscriber::set_global_default(subscriber::NoSubscriber::new())?;
    Ok(())
}

#[cfg_attr(
    any(not(feature = "cloud_services"), target_family = "wasm"),
    derive(Default)
)]
pub struct Initialization {
    initialization_warning: Option<anyhow::Error>,
    #[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
    active_spans: Option<native::ActiveSpanRegistry>,
    #[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
    provider: Option<opentelemetry_sdk::trace::SdkTracerProvider>,
    #[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
    shutdown_timeout: std::time::Duration,
}

#[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
impl Default for Initialization {
    fn default() -> Self {
        Self {
            initialization_warning: None,
            active_spans: None,
            provider: None,
            shutdown_timeout: DEFAULT_EXPORT_TIMEOUT,
        }
    }
}

impl Initialization {
    pub fn log_initialization_warning(&mut self) {
        if let Some(err) = self.initialization_warning.take() {
            log::warn!("Failed to initialize cloud-agent OpenTelemetry exporting: {err:#}");
        }
    }

    pub(crate) fn shutdown(&mut self) {
        #[cfg(all(feature = "cloud_services", not(target_family = "wasm")))]
        {
            match (self.active_spans.take(), self.provider.take()) {
                (Some(active_spans), Some(provider)) => {
                    if let Err(err) = active_spans.shutdown(&provider, self.shutdown_timeout) {
                        log::warn!(
                            "Failed to shut down cloud-agent OpenTelemetry exporting: {err}"
                        );
                    }
                }
                (None, Some(provider)) => {
                    if let Err(err) = provider.shutdown_with_timeout(self.shutdown_timeout) {
                        log::warn!(
                            "Failed to shut down cloud-agent OpenTelemetry exporting: {err}"
                        );
                    }
                }
                (Some(_), None) | (None, None) => {}
            }
        }
    }
}

impl Drop for Initialization {
    fn drop(&mut self) {
        self.shutdown();
    }
}
