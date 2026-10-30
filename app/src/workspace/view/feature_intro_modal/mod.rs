mod view;

#[cfg(feature = "agent_runtime")]
pub use view::init;
pub use view::{FEATURE_INTROS, FeatureIntroId};
#[cfg(feature = "agent_runtime")]
pub use view::{
    FeatureIntroCtaTarget, FeatureIntroModal, FeatureIntroModalEvent, feature_intro_by_id,
};
