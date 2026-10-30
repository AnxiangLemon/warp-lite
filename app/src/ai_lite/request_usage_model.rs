use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use warpui::{Entity, ModelContext, SingletonEntity};

use crate::server::server_api::ai::AIClient;

pub use super::credit_availability::{BonusGrant, BonusGrantScope};

#[derive(Clone, Debug, Default)]
pub enum BuyCreditsBannerDisplayState {
    #[default]
    Hidden,
}

#[derive(Clone, Debug)]
pub enum AIRequestUsageModelEvent {}

#[derive(Clone, Debug, Default)]
pub struct RequestUsageInfo;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum RequestLimitRefreshDuration {
    #[default]
    Weekly,
    Monthly,
    EveryTwoWeeks,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct RequestLimitInfo {
    pub limit: usize,
    pub requests_used_since_last_refresh: usize,
    pub next_refresh_time: DateTime<Utc>,
    pub is_unlimited: bool,
    pub voice_request_limit: usize,
    pub voice_requests_used_since_last_refresh: usize,
    pub is_unlimited_voice: bool,
}

pub struct AIRequestUsageModel;

#[derive(Clone, Copy, Debug, Default)]
pub struct CodebaseContextUsageLimit {
    pub max_indices_allowed: usize,
    pub max_files_per_repo: usize,
    pub embedding_generation_batch_size: usize,
}

impl AIRequestUsageModel {
    pub fn new(_ai_client: Arc<dyn AIClient>, _ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn request_availability_refresh(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn codebase_context_limits(&self) -> CodebaseContextUsageLimit {
        CodebaseContextUsageLimit::default()
    }
}

impl Entity for AIRequestUsageModel {
    type Event = ();
}

impl SingletonEntity for AIRequestUsageModel {}
