use serde::{Deserialize, Serialize};
use warp_cli::agent::Harness;
use warpui::{Entity, ModelContext, SingletonEntity};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HarnessModelInfo {
    pub id: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_level: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HarnessAvailability {
    pub harness: Harness,
    pub display_name: String,
    pub enabled: bool,
    #[serde(default)]
    pub available_models: Vec<HarnessModelInfo>,
}

#[derive(Clone, Debug)]
pub enum HarnessAvailabilityEvent {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AuthSecretFetchState {
    #[default]
    Unknown,
}

pub struct HarnessAvailabilityModel;

impl HarnessAvailabilityModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for HarnessAvailabilityModel {
    type Event = HarnessAvailabilityEvent;
}

impl SingletonEntity for HarnessAvailabilityModel {}
