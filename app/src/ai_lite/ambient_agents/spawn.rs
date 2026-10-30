use std::time::Duration;

use futures::Stream;
use session_sharing_protocol::common::SessionId;

pub use super::AmbientAgentEvent;
use super::AmbientAgentTaskId;
use crate::server::server_api::ai::{AIClient, SpawnAgentRequest};

#[derive(Clone, Debug, Default)]
pub struct SessionJoinInfo;

pub fn spawn_task(
    _request: SpawnAgentRequest,
    _ai_client: std::sync::Arc<dyn AIClient>,
    _timeout: Option<Duration>,
) -> impl Stream<Item = Result<AmbientAgentEvent, anyhow::Error>> {
    futures::stream::empty()
}

pub fn monitor_spawned_task(
    _task_id: AmbientAgentTaskId,
    _run_id: String,
    _at_capacity: bool,
    _ai_client: std::sync::Arc<dyn AIClient>,
    _timeout: Option<Duration>,
) -> impl Stream<Item = Result<AmbientAgentEvent, anyhow::Error>> {
    futures::stream::empty()
}

pub fn submit_run_followup(
    _message: String,
    _run_id: AmbientAgentTaskId,
    _previous_session_id: Option<SessionId>,
    _ai_client: std::sync::Arc<dyn AIClient>,
    _timeout: Option<Duration>,
) -> impl Stream<Item = Result<AmbientAgentEvent, anyhow::Error>> {
    futures::stream::empty()
}
