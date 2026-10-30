mod legacy;
mod maa;
mod static_prompt_suggestions;

#[cfg(feature = "agent_runtime")]
pub use legacy::PassiveSuggestionsEvent as LegacyPassiveSuggestionsEvent;
pub use legacy::PassiveSuggestionsModel as LegacyPassiveSuggestionsModel;
#[cfg(feature = "agent_runtime")]
pub use maa::PassiveSuggestionsEvent as MaaPassiveSuggestionsEvent;
pub use maa::PassiveSuggestionsModel as MaaPassiveSuggestionsModel;
use warpui::ModelHandle;

#[derive(Clone)]
pub struct PassiveSuggestionsModels {
    pub legacy: ModelHandle<LegacyPassiveSuggestionsModel>,
    pub maa: ModelHandle<MaaPassiveSuggestionsModel>,
}
