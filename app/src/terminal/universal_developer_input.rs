pub enum AtContextMenuDisabledReason {
    #[cfg(target_family = "wasm")]
    Wasm,
    #[cfg(not(target_family = "wasm"))]
    NoObjectsAvailable,
    #[cfg(not(target_family = "wasm"))]
    SshWithoutRemoteServer,
    #[cfg(not(target_family = "wasm"))]
    Subshell,
    #[cfg(not(target_family = "wasm"))]
    DisabledInTerminalMode,
}

#[cfg(feature = "agent_runtime")]
include!("universal_developer_input_runtime.rs");
