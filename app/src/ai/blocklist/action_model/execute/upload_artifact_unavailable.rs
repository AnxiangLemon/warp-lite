use futures::FutureExt;
use futures::future::BoxFuture;
use warpui::{Entity, EntityId, ModelContext, ModelHandle};

use super::{ActionExecution, AnyActionExecution, ExecuteActionInput, PreprocessActionInput};
use crate::ai::agent::{AIAgentActionResultType, UploadArtifactResult};
use crate::terminal::model::session::active_session::ActiveSession;

pub struct UploadArtifactExecutor;

impl UploadArtifactExecutor {
    pub fn new(active_session: ModelHandle<ActiveSession>, terminal_view_id: EntityId) -> Self {
        let _ = (active_session, terminal_view_id);
        Self
    }

    pub(super) fn should_autoexecute(
        &self,
        input: ExecuteActionInput,
        ctx: &mut ModelContext<Self>,
    ) -> bool {
        let _ = (input, ctx);
        false
    }

    pub(super) fn execute(
        &mut self,
        input: ExecuteActionInput,
        ctx: &mut ModelContext<Self>,
    ) -> AnyActionExecution {
        let _ = (input, ctx);
        ActionExecution::<()>::Sync(AIAgentActionResultType::UploadArtifact(
            UploadArtifactResult::Error(
                "Agent artifact uploads are unavailable in this build".to_owned(),
            ),
        ))
        .into()
    }

    pub(super) fn preprocess_action(
        &mut self,
        input: PreprocessActionInput,
        ctx: &mut ModelContext<Self>,
    ) -> BoxFuture<'static, ()> {
        let _ = (input, ctx);
        futures::future::ready(()).boxed()
    }
}

impl Entity for UploadArtifactExecutor {
    type Event = ();
}
