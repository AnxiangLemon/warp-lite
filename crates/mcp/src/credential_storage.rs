use serde::Serialize;
use serde::de::DeserializeOwned;
use warp_errors::report_error;
use warpui_extras::secure_storage::AppContextExt as _;

/// Loads credentials from secure storage at the provided key.
pub fn load_credentials_from_secure_storage<T: DeserializeOwned + Default>(
    app: &mut warpui::AppContext,
    key: &str,
) -> T {
    app.secure_storage()
        .read_value(key)
        .inspect_err(|err| {
            if !matches!(err, warpui_extras::secure_storage::Error::NotFound) {
                log::warn!("Failed to read MCP credentials from secure storage: {err:#}");
            }
        })
        .ok()
        .and_then(|value| serde_json::from_str(&value).ok())
        .unwrap_or_default()
}

/// Writes credentials to secure storage at the provided key.
pub fn write_to_secure_storage<T: Serialize>(
    app: &mut warpui::AppContext,
    key: &str,
    credentials: &T,
) {
    match serde_json::to_string(credentials) {
        Ok(json) => {
            if let Err(err) = app.secure_storage().write_value(key, &json) {
                report_error!(
                    anyhow::Error::new(err)
                        .context("Failed to write MCP credentials to secure storage")
                );
            }
        }
        Err(err) => {
            report_error!(
                anyhow::Error::new(err)
                    .context("Failed to serialize MCP credentials for secure storage")
            );
        }
    }
}
