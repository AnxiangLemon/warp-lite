pub struct HarnessConfig;
pub struct HarnessModelConfig;
pub struct HarnessAuthSecretsConfig;
pub struct AttachmentInput;
pub struct TaskAttachment;

pub fn normalize_orchestrator_agent_name(name: &str) -> String {
    name.to_owned()
}

pub fn cancel_task_with_toast<V: warpui::View>(
    _task_id: super::AmbientAgentTaskId,
    _ctx: &mut warpui::ViewContext<V>,
) {
}

pub fn cancel_task_silently(
    _task_id: super::AmbientAgentTaskId,
    _ctx: &mut warpui::AppContext,
) {
}
