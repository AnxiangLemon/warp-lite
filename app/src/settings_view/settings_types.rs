use serde::{Deserialize, Serialize};

use crate::server::ids::SyncId;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum EnvironmentsPage {
    #[default]
    List,
    Edit {
        env_id: SyncId,
    },
    Create,
}

/// TeamsInviteOption is whether the user is looking at invite-by-link or invite-by-email.
#[derive(Clone, PartialEq, Eq, Debug, Default, Copy, Serialize, Deserialize)]
pub enum TeamsInviteOption {
    #[default]
    Link,
    Email,
}

impl std::fmt::Display for TeamsInviteOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Link => "Link",
            Self::Email => "Email",
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenTeamsSettingsModalArgs {
    pub invite_email: Option<String>,
}
