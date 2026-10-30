use chrono::{DateTime, Local};
use warpui::{EntityId, WindowId};

use crate::ai::agent::api::ServerConversationToken;
use crate::ai::agent::conversation::{AIConversationId};
use crate::workspace::PaneViewLocator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationNavigationData {
    pub id: AIConversationId,
    pub title: String,
    pub initial_query: Option<String>,
    pub last_updated: DateTime<Local>,
    pub terminal_view_id: Option<EntityId>,
    pub window_id: Option<WindowId>,
    pub pane_view_locator: Option<PaneViewLocator>,
    pub initial_working_directory: Option<String>,
    pub latest_working_directory: Option<String>,
    pub is_selected: bool,
    pub is_in_active_pane: bool,
    pub is_closed: bool,
    pub server_conversation_token: Option<ServerConversationToken>,
}
