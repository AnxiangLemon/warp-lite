use std::collections::HashMap;
use std::ffi::OsString;
use std::sync::Arc;

use crate::ai::ambient_agents::AmbientAgentTaskId;
use crate::server::server_api::ai::AIClient;
use crate::terminal::shell::ShellType;
use warp_cli::agent::Harness;

#[derive(Clone)]
pub(super) struct PreparedLocalHarnessLaunch {
    pub command: String,
    pub env_vars: HashMap<OsString, OsString>,
    pub run_id: String,
    pub task_id: AmbientAgentTaskId,
}

pub(super) async fn prepare_local_harness_child_launch(
    _harness: Harness,
    _shell_type: ShellType,
    _ai_client: Arc<dyn AIClient>,
) -> Result<PreparedLocalHarnessLaunch, anyhow::Error> {
    Err(anyhow::anyhow!("local harness launch unavailable in warp_lite"))
}
