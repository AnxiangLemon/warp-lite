use futures::StreamExt;
use futures::executor::block_on;
use oauth2::AsyncHttpClient;

use super::*;

#[test]
fn offline_requests_fail_before_running_request_hooks() {
    let mut client = Client::new_offline();
    client.set_before_request_fn(Box::new(|_, _| {
        panic!("offline requests must not reach hooks")
    }));
    client.set_after_response_fn(Box::new(|_| {
        panic!("offline requests cannot have responses")
    }));

    let request = client
        .post("https://example.invalid/unused")
        .header("X-Test", "offline")
        .basic_auth("user", Some("password"))
        .bearer_auth("token")
        .timeout(Duration::from_secs(1))
        .json(&serde_json::json!({"offline": true}))
        .prevent_sleep("offline test");
    let result = block_on(request.send());

    assert!(client.is_offline());
    assert!(result.is_err_and(|error| error.is_builder()));
}

#[test]
fn offline_event_source_reports_one_error_then_ends() {
    let client = Client::new_offline();
    let mut stream = client.get("https://example.invalid/events").eventsource();

    let first = block_on(stream.next());
    let second = block_on(stream.next());

    assert!(
        matches!(first, Some(Err(reqwest_eventsource::Error::Transport(error))) if error.is_builder())
    );
    assert!(second.is_none());
}

#[test]
fn offline_oauth_fails_without_initializing_a_transport() {
    let client = Client::new_offline();
    let request = http::Request::builder()
        .uri("https://example.invalid/oauth")
        .body(Vec::new())
        .unwrap();

    assert!(block_on(client.call(request)).is_err());
}

#[cfg(not(feature = "network_transport"))]
#[test]
fn default_client_cannot_construct_or_send_a_network_request() {
    let client = Client::default();

    let result = block_on(client.get("https://example.invalid/unused").send());

    assert!(client.is_offline());
    assert!(result.is_err_and(|error| error.is_builder()));
}
