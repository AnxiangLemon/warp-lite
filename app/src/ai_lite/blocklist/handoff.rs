use std::path::PathBuf;
use std::sync::Arc;

use warpui::AppContext;

pub struct PendingCloudLaunch;
pub struct HandoffLaunchAttachments;
pub struct HandoffPrepareInput;
pub struct HandoffPresentationSnapshot;
pub struct HandoffRestoration;
pub struct HandoffTargetMaterialization;
pub struct MaterializeHandoffTarget;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HandoffPrepareError {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HandoffCommitOutcome {
    #[default]
    Unknown,
}

pub fn suggest_handoff_environment(_app: &AppContext) -> Option<PendingCloudLaunch> {
    None
}

pub fn prepare_handoff(
    _input: HandoffPrepareInput,
    _app: &AppContext,
) -> Result<HandoffPresentationSnapshot, HandoffPrepareError> {
    Err(HandoffPrepareError::Unknown)
}

pub fn execute_handoff(_app: &AppContext) -> HandoffCommitOutcome {
    HandoffCommitOutcome::Unknown
}

use crate::ai::orchestration::CloudAgentStartupIssue;
use crate::server::server_api::ai::SpawnAgentRequest;

pub struct HandoffCommitFailure;
pub struct HandoffSnapshotUploadError;

pub struct HandoffCreated {
    pub task_id: crate::ai::ambient_agents::AmbientAgentTaskId,
    pub run_id: String,
    pub url: String,
    pub at_capacity: bool,
    pub request: SpawnAgentRequest,
    pub derived_workspace_had_content: bool,
    pub snapshot_failed: bool,
}

pub fn handoff_dispatch_error(_issue: &CloudAgentStartupIssue) -> String {
    String::new()
}

pub fn materialize_handoff_target(
    _app: &AppContext,
) -> Option<HandoffTargetMaterialization> {
    None
}

pub mod touched_repos {
    use std::path::PathBuf;

    #[derive(Clone, Debug, Default)]
    pub struct TouchedWorkspace {
        pub repos: Vec<TouchedRepo>,
        pub orphan_files: Vec<PathBuf>,
    }

    #[derive(Clone, Debug)]
    pub struct TouchedRepo {
        pub git_root: PathBuf,
    }

    pub async fn derive_touched_workspace(_paths: Vec<PathBuf>) -> TouchedWorkspace {
        TouchedWorkspace::default()
    }
}

pub mod snapshot {
    use std::sync::Arc;

    use session_sharing_protocol::common::SessionId;
    use warpui::ViewContext;

    use crate::server::server_api::ai::AIClient;
    use crate::terminal::model::session::SessionId as TerminalSessionId;
    use crate::workspace::Workspace;

    pub enum SnapshotUploadTarget {
        Local {
            ai_client: Arc<dyn AIClient>,
            http: Arc<http_client::Client>,
        },
        Remote {
            handle: crate::remote_server::manager::HostRequestHandle,
        },
    }

    pub fn resolve_upload_target(
        _session_id: TerminalSessionId,
        ctx: &mut ViewContext<Workspace>,
    ) -> SnapshotUploadTarget {
        let server_api_provider = crate::server::server_api::ServerApiProvider::as_ref(ctx);
        SnapshotUploadTarget::Local {
            ai_client: server_api_provider.get_ai_client(),
            http: server_api_provider.get_http_client(),
        }
    }

    pub fn upload_result_to_proto() {}
}

pub use touched_repos::derive_touched_workspace;
