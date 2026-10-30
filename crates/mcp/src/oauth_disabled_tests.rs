use serde_json::json;

use super::PersistedCredentials;

#[test]
fn disabled_transport_preserves_serialized_credentials() {
    let value = json!({
        "client_id": "test-client",
        "token_response": { "access_token": "test-token", "token_type": "Bearer" },
        "client_secret": null,
    });
    let credentials: PersistedCredentials = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(credentials).unwrap(), value);
}
