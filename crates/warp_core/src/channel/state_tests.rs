use super::derive_http_origin_from_ws_url;

#[test]
fn data_profiles_cannot_escape_the_application_directory() {
    for profile in [
        "",
        ".",
        "..",
        "../other",
        r"..\other",
        "C:\\other",
        "/tmp",
        "a/b",
        "a\\b",
    ] {
        assert!(!super::valid_data_profile(profile), "{profile:?}");
    }
}

#[test]
fn data_profiles_accept_bounded_portable_names() {
    assert!(super::valid_data_profile("Startup-before_20261031"));
    assert!(super::valid_data_profile(&"a".repeat(64)));
    assert!(!super::valid_data_profile(&"a".repeat(65)));
}

#[test]
fn wss_becomes_https_and_strips_path() {
    let got = derive_http_origin_from_ws_url("wss://rtc.app.warp.dev/graphql/v2");
    assert_eq!(got.as_deref(), Some("https://rtc.app.warp.dev"));
}

#[test]
fn ws_becomes_http_and_preserves_port() {
    let got = derive_http_origin_from_ws_url("ws://localhost:8080/graphql/v2");
    assert_eq!(got.as_deref(), Some("http://localhost:8080"));
}

#[test]
fn unparseable_input_returns_none() {
    assert!(derive_http_origin_from_ws_url("not a url").is_none());
    assert!(derive_http_origin_from_ws_url("https://app.warp.dev").is_none());
}
