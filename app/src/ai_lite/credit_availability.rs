use chrono::{DateTime, Utc};
use warp_graphql::billing::BonusGrantType;

#[derive(Clone, Debug, Default)]
pub enum AICreditAvailability {
    #[default]
    Unknown,
}

#[derive(Clone, Debug)]
pub struct BonusGrant {
    pub created_at: DateTime<Utc>,
    pub cost_cents: i32,
    pub expiration: Option<DateTime<Utc>>,
    pub grant_type: BonusGrantType,
    pub reason: String,
    pub user_facing_message: Option<String>,
    pub request_credits_granted: i32,
    pub request_credits_remaining: i32,
    pub scope: BonusGrantScope,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BonusGrantScope {
    User,
    Team(crate::workspaces::workspace::WorkspaceUid),
    Workspace(crate::workspaces::workspace::WorkspaceUid),
}
