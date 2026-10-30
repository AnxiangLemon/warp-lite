//! Minimal AI surface for `warp_lite` builds. Full Agent Mode lives in `app/src/ai/`.

pub mod active_agent_views_model;
pub mod agent;
pub mod agent_conversations_model;
pub mod agent_management;
#[cfg(not(target_family = "wasm"))]
pub mod agent_sdk;
pub mod agent_tips;
pub mod ambient_agents;
pub mod artifacts;
pub mod attachment_utils;
pub mod block_context;
pub mod blocklist;
pub mod cloud_agent_config;
pub mod cloud_environments;
pub mod codebase_auto_indexing;
pub mod connected_self_hosted_workers;
pub mod conversation_details_panel;
pub mod conversation_status_ui;
pub mod conversation_utils;
pub mod credit_availability;
pub mod custom_model_router_editor;
pub mod custom_model_routers;
pub mod document;
pub mod execution_profiles;
pub mod facts;
pub mod generate_block_title;
pub mod generate_code_review_content;
pub mod get_relevant_files;
pub mod harness_availability;
pub mod llms;
pub mod loading;
pub mod mcp;
pub mod metadata_project_rules;
pub mod outline;
pub mod persisted_workspace;
pub mod predict;
pub mod pricing_promotion;
pub mod remote_agent_context;
pub mod request_usage_model;
pub mod restored_conversations;
pub mod skills;
pub mod voice;

#[cfg(not(target_family = "wasm"))]
pub mod aws_credentials;
#[cfg(not(target_family = "wasm"))]
pub mod geap_credentials;
#[cfg(not(target_family = "wasm"))]
pub mod tui_api_keys;

pub mod auth_secret_types;
pub mod cloud_agent_settings;
pub mod conversation_export;
pub mod conversation_navigation;
pub mod conversation_rename;
pub mod harness_display;
pub mod local_harness_setup;
pub mod onboarding;
pub mod orchestration;

pub use request_usage_model::AIRequestUsageModel;
pub use request_usage_model::AIRequestUsageModelEvent;
pub use request_usage_model::BuyCreditsBannerDisplayState;
pub use request_usage_model::{RequestLimitInfo, RequestLimitRefreshDuration, RequestUsageInfo};
pub use credit_availability::{AICreditAvailability, BonusGrant, BonusGrantScope};
pub use agent_tips::AgentTip;

pub use ai::document::{AIDocumentId, AIDocumentVersion};

pub fn init(_app: &mut warpui::AppContext) {}
