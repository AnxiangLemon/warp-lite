use clap::Parser;

use super::Args;

#[test]
fn lite_cli_preserves_local_app_urls_and_parent_process() {
    let args =
        Args::try_parse_from(["warp-lite", "--parent-pid=42", "warp://action/new_tab"]).unwrap();
    assert_eq!(args.app.parent.pid, Some(42));
    assert_eq!(args.app.urls[0].as_str(), "warp://action/new_tab");
}

#[test]
fn lite_cli_rejects_cloud_commands_and_credentials() {
    assert!(Args::try_parse_from(["warp-lite", "agent", "run"]).is_err());
    assert!(Args::try_parse_from(["warp-lite", "login"]).is_err());
    assert!(Args::try_parse_from(["warp-lite", "--api-key", "unused"]).is_err());
    assert!(Args::try_parse_from(["warp-lite", "--warpctrl"]).is_err());
}

#[cfg(all(unix, feature = "local_tty"))]
#[test]
fn lite_cli_preserves_the_local_terminal_worker_protocol() {
    let args = Args::try_parse_from(["warp-lite", "terminal-server", "--parent-pid=42"]).unwrap();
    let Some(super::Worker::TerminalServer(worker)) = args.worker else {
        panic!("expected a local terminal worker");
    };
    assert_eq!(worker.parent.pid, Some(42));
}

#[test]
fn lite_cli_preserves_local_graphics_diagnostics() {
    assert!(
        Args::try_parse_from(["warp-lite", "--dump-debug-info"])
            .unwrap()
            .dump_debug_info
    );
}
