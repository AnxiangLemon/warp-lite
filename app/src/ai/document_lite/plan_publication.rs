use warpui::{Entity, ModelContext};

use crate::ai::agent::conversation::AIConversationId;

pub(in crate::ai) struct PendingPlanPublication {}

pub(in crate::ai) fn prepare_plan_publications<E: Entity>(
    _conversation_id: AIConversationId,
    _ctx: &mut ModelContext<E>,
) -> Vec<PendingPlanPublication> {
    vec![]
}

pub(in crate::ai) async fn wait_for_plan_publications(_pending: Vec<PendingPlanPublication>) {}
