pub mod driver {
    pub const WARP_DRIVE_SYNC_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(0);

    pub fn snapshot_upload_timeout() -> std::time::Duration {
        std::time::Duration::from_secs(0)
    }

    #[derive(Debug)]
    pub enum AgentDriverError {
        Failed,
    }

    impl std::fmt::Display for AgentDriverError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "agent driver error")
        }
    }

    impl std::error::Error for AgentDriverError {}

    pub mod environment {
        #[derive(Debug, thiserror::Error)]
        pub enum PrepareEnvironmentError {
            #[error("prepare environment unavailable in warp_lite")]
            Unavailable,
        }

        pub async fn prepare_environment() -> Result<(), PrepareEnvironmentError> {
            Err(PrepareEnvironmentError::Unavailable)
        }
    }

    pub mod terminal {
        pub struct TerminalDriver;
    }

    pub mod harness {
        pub struct HarnessDriver;

        pub mod claude_code {
            pub fn prepare_claude_environment_config() {}
        }

        pub mod claude_transcript {
            use std::io::Read;

            pub struct ClaudeLocalContinuation;

            pub fn rehydrate_claude_transcript_from_reader(
                _reader: impl Read,
            ) -> anyhow::Result<ClaudeLocalContinuation> {
                Err(anyhow::anyhow!(
                    "rehydrate_claude_transcript_from_reader unavailable in warp_lite"
                ))
            }
        }

        pub mod codex_transcript {
            use std::io::Read;

            pub struct CodexLocalContinuation;

            pub fn rehydrate_codex_transcript_from_reader(
                _reader: impl Read,
            ) -> anyhow::Result<CodexLocalContinuation> {
                Err(anyhow::anyhow!(
                    "rehydrate_codex_transcript_from_reader unavailable in warp_lite"
                ))
            }
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum HarnessKind {
            #[default]
            Unknown,
        }

        pub fn harness_kind(_harness: warp_cli::agent::Harness) -> HarnessKind {
            HarnessKind::Unknown
        }

        pub fn harness_model_env_vars() -> std::collections::HashMap<String, String> {
            std::collections::HashMap::new()
        }

        pub fn remove_claude_externally_managed_listener_env_vars(
            _env: &mut std::collections::HashMap<std::ffi::OsString, std::ffi::OsString>,
        ) {
        }

        pub fn auth_check_command_for(_harness: warp_cli::agent::Harness) -> Option<String> {
            None
        }
    }

    pub async fn upload_snapshot_for_handoff(
        _repo_paths: Vec<std::path::PathBuf>,
        _orphan_file_paths: Vec<std::path::PathBuf>,
        _client: std::sync::Arc<dyn crate::server::server_api::ai::AIClient>,
        _http: &http_client::Client,
    ) -> anyhow::Result<Option<crate::server::server_api::ai::InitialSnapshotToken>> {
        Ok(None)
    }
}

pub mod retry {
    use std::future::Future;

    use anyhow::Result;

    pub async fn with_bounded_retry<T, F, Fut>(operation: &str, mut attempt_fn: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let _ = operation;
        attempt_fn().await
    }
}

pub mod task_env_vars {}

pub mod setup_observability {
    pub struct SetupClientEventReporter;
}

pub fn validate_cli_installed() -> bool {
    false
}

pub fn run(
    _ctx: &mut warpui::AppContext,
    _command: warp_cli::agent::AgentCommand,
    _global_options: warp_cli::GlobalOptions,
) -> Result<(), anyhow::Error> {
    Ok(())
}
