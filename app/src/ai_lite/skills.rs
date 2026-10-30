use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SkillOpenOrigin {
    #[default]
    Unknown,
}

#[derive(Clone, Debug)]
pub enum SkillTelemetryEvent {}

#[derive(Clone, Debug)]
pub enum SkillManagerEvent {}

pub struct BundledSkill;
pub struct SkillDescriptor;
pub struct SkillManager;

impl SkillManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for SkillManager {
    type Event = SkillManagerEvent;
}

impl SingletonEntity for SkillManager {}

pub fn bundled_skill_snapshot_protos() -> Vec<()> {
    vec![]
}
