pub use ai::{LLMId, LLMProvider};

use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LLMModelHost {
    #[default]
    Unknown,
}

#[derive(Clone, Debug)]
pub enum LLMPreferencesEvent {}

pub struct LLMPreferences;

impl LLMPreferences {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for LLMPreferences {
    type Event = LLMPreferencesEvent;
}

impl SingletonEntity for LLMPreferences {}

pub fn should_show_key_icon_for_model(_model_id: &LLMId) -> bool {
    false
}

pub fn dedupe_model_display_names<'a>(
    models: impl IntoIterator<Item = &'a LLMInfo>,
) -> Vec<&'a LLMInfo> {
    models.into_iter().collect()
}

pub struct AvailableLLMs;
pub struct LLMInfo {
    pub id: LLMId,
    pub display_name: String,
}

impl LLMInfo {
    pub fn base_model_name(&self) -> &str {
        &self.display_name
    }

    pub fn has_reasoning_level(&self) -> bool {
        false
    }
}

pub struct LLMSpec;
pub struct LLMContextWindow;
pub struct LLMUsageMetadata {
    pub credit_multiplier: Option<f32>,
}

pub struct ModelsByFeature;
pub struct RoutingHostConfig;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DisableReason {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ByoKeySource {
    #[default]
    Unknown,
}

pub fn byo_key_source_for_model(_model_id: &LLMId) -> ByoKeySource {
    ByoKeySource::Unknown
}

pub struct ModelConfigError {
    pub message: String,
}

pub struct ModelIconFlags;

pub fn model_leading_icon(
    _llm: &LLMInfo,
    _flags: ModelIconFlags,
    _app: &warpui::AppContext,
) -> Option<warp_core::ui::Icon> {
    None
}

pub fn should_show_bedrock_icon_for_model(_model_id: &LLMId) -> bool {
    false
}

pub fn should_show_gemini_enterprise_agent_platform_icon_for_model(_model_id: &LLMId) -> bool {
    false
}
