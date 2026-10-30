use std::io::ErrorKind;
use std::net::TcpListener;

use futures::executor::block_on;

use crate::WebSocket;

#[test]
fn disabled_transport_does_not_open_a_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let url = format!("ws://{}/graphql", listener.local_addr().unwrap());
    let result = block_on(WebSocket::connect_with_headers(
        &url,
        ["graphql-transport-ws"],
        vec![("Authorization", "Bearer test-only".to_owned())],
    ));
    let error = result.err().expect("transport must be disabled");
    assert_eq!(
        error.to_string(),
        "WebSocket transport is disabled in this build"
    );
    assert_eq!(listener.accept().unwrap_err().kind(), ErrorKind::WouldBlock);
}

#[test]
fn disabled_transport_rejects_tls_without_a_runtime() {
    let result = block_on(WebSocket::connect("wss://localhost/graphql", None::<&str>));
    let error = result.err().expect("transport must be disabled");
    assert_eq!(
        error.to_string(),
        "WebSocket transport is disabled in this build"
    );
}
