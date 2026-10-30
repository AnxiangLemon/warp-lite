//! Lite stubs for AI-only fields and methods on [`super::Input`].

use warp_core::ui::appearance::Appearance;
use warpui::elements::Element;
use warpui::{AppContext, ModelHandle, SingletonEntity, ViewContext, ViewHandle};

use super::Input;
use crate::ai::agent::conversation::AIConversationId;
use crate::ai::ambient_agents::telemetry::HandoffEntryPoint;
use crate::terminal::input::models::InlineModelSelectorTab;
use crate::editor::EditOrigin;
use crate::terminal::input::InputPrefixMode;
use crate::terminal::view::ambient_agent::AmbientAgentViewModel;
use crate::terminal::view::queued_prompts_panel::QueuedPromptsPanelView;

impl Input {
    pub(crate) fn attach_ambient_agent_view_model(
        &mut self,
        _view_model: ModelHandle<AmbientAgentViewModel>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal) fn auth_secret_delete_confirmation_dialog_element(
        &self,
        _ctx: &AppContext,
    ) -> Option<Box<dyn Element>> {
        None
    }

    pub(in crate::terminal) fn auth_secret_ftux_view(
        &self,
    ) -> Option<&ViewHandle<crate::terminal::view::ambient_agent::AuthSecretFtuxView>> {
        None
    }

    pub(super) fn open_v2_host_selector(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(super) fn open_v2_harness_selector(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(super) fn open_v2_environment_selector(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_is_processing_attached_images(
        &mut self,
        _is_processing_attached_images: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn clear_cached_hint_text(&mut self) {}

    fn predict_am_query(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(in crate::terminal::input) fn render_attachment_chips(
        &self,
        _appearance: &Appearance,
    ) -> Option<Box<dyn Element>> {
        None
    }

    pub fn update_image_context_options(&mut self, ctx: &mut ViewContext<Self>) {
        use crate::ai::agent::AIAgentContext;
        use crate::ai::blocklist::InputType;
        use crate::editor::ImageContextOptions;
        use crate::features::FeatureFlag;

        let ai_input_model = self.ai_input_model.as_ref(ctx);
        let llm_prefs = crate::ai::llms::LLMPreferences::as_ref(ctx);
        let vision_supported = llm_prefs.vision_supported(ctx, Some(self.terminal_view_id));
        let num_images_attached = self.ai_context_model.as_ref(ctx).pending_images().len();
        let conversation = self.ai_context_model.as_ref(ctx).selected_conversation(ctx);
        let num_images_in_conversation = conversation
            .and_then(|conversation| conversation.get_root_task())
            .into_iter()
            .flat_map(|task| {
                task.all_contexts()
                    .filter(|context| matches!(context, AIAgentContext::Image(_)))
            })
            .count();

        let image_context_options = if FeatureFlag::ImageAsContext.is_enabled()
            && matches!(ai_input_model.input_type(), InputType::AI)
        {
            ImageContextOptions::Enabled {
                unsupported_model: !vision_supported,
                is_processing_attached_images: false,
                num_images_attached,
                num_images_in_conversation,
            }
        } else {
            ImageContextOptions::Disabled
        };

        self.editor.update(ctx, move |editor, ctx| {
            editor.update_image_context_options(image_context_options, ctx);
            ctx.notify();
        });
    }

    pub(crate) fn queued_prompts_panel(
        &self,
    ) -> Option<&ViewHandle<QueuedPromptsPanelView>> {
        None
    }

    pub(crate) fn is_queued_prompt_inline_editor_focused(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub(in crate::terminal) fn is_editing_queued_prompt(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub(in crate::terminal) fn is_agent_view_active(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub(in crate::terminal) fn is_agent_view_fullscreen(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub(in crate::terminal) fn is_agent_view_inline(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub(in crate::terminal) fn is_cloud_agent_pre_first_exchange(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub(in crate::terminal) fn prefix_mode(&self, ctx: &AppContext) -> InputPrefixMode {
        let ai_input_model = self.ai_input_model.as_ref(ctx);
        let is_shell_active = ai_input_model.is_input_type_locked()
            && !ai_input_model.input_type().is_ai();
        if is_shell_active {
            InputPrefixMode::Shell
        } else {
            InputPrefixMode::None
        }
    }

    pub(in crate::terminal) fn activate_cloud_handoff_compose(
        &mut self,
        _entry_point: HandoffEntryPoint,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(in crate::terminal) fn exit_cloud_handoff_compose(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(in crate::terminal) fn can_activate_cloud_handoff_prefix(
        &self,
        _edit_origin: &EditOrigin,
        _ctx: &AppContext,
    ) -> bool {
        false
    }

    pub(in crate::terminal) fn maybe_activate_cloud_handoff_prefix(
        &mut self,
        _edit_origin: &EditOrigin,
        _ctx: &mut ViewContext<Self>,
    ) -> bool {
        false
    }

    pub(super) fn open_conversation_menu(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(super) fn open_skill_selector(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(super) fn open_invoke_skill_selector(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(super) fn open_model_selector_and_snapshot_prompt(
        &mut self,
        _initial_tab: InlineModelSelectorTab,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn open_profile_selector(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(super) fn open_prompts_menu(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn open_plan_menu(
        &mut self,
        _conversation_id: AIConversationId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn open_rewind_menu(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(crate) fn handoff_entry_point(&self, _ctx: &AppContext) -> HandoffEntryPoint {
        HandoffEntryPoint::FooterChip
    }

    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pub(crate) fn exit_cloud_handoff_compose_and_clear_prompt(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    #[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
    pub(crate) fn restore_cloud_handoff_draft(
        &mut self,
        _launch: crate::ai::blocklist::handoff::PendingCloudLaunch,
        _environment_id: Option<crate::server::ids::SyncId>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}
