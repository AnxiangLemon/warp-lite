use warpui::r#async::BoxFuture;

use super::*;

fn static_auth_context() -> Arc<RemoteServerAuthContext> {
    Arc::new(RemoteServerAuthContext::new(
        || -> BoxFuture<'static, Option<String>> { Box::pin(async { None }) },
        || "user id/with spaces".to_string(),
        String::new(),
        String::new(),
        true,
    ))
}

#[test]
fn remote_proxy_command_quotes_identity_key() {
    let transport = SshTransport::new(
        PathBuf::from("/tmp/control-master.sock"),
        static_auth_context(),
        true,
    );

    let command = transport.remote_proxy_command();

    assert!(command.contains("remote-server-proxy --identity-key"));
    assert!(command.contains("'user id/with spaces'"));
}

#[cfg(not(feature = "cloud_services"))]
#[test]
fn offline_remote_install_does_not_open_the_ssh_control_socket() {
    let transport = SshTransport::new(
        PathBuf::from("nonexistent/control-master.sock"),
        static_auth_context(),
        true,
    );

    let outcome = futures::executor::block_on(transport.install_binary());

    assert!(outcome.source.is_none());
    assert_eq!(
        outcome
            .result
            .expect_err("offline installation must fail immediately")
            .to_string(),
        "Remote server installation is unavailable in this build"
    );
}
