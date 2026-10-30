use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AIConversationId(Uuid);

impl AIConversationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusColorStyle {
    #[default]
    Unknown,
}

pub struct AIAgentHarness;

#[derive(Clone, Debug, Default)]
pub struct AIAgentConversationFormat;

#[derive(Clone, Debug, Default)]
pub struct ServerAIConversationMetadata;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AIAgentSerializedBlockFormat {
    #[default]
    Unknown,
}

impl Display for AIConversationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AIConversationId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl TryFrom<String> for AIConversationId {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AIConversationAutoexecuteMode {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConversationStatus {
    #[default]
    Idle,
    Running,
    Completed,
}

#[derive(Clone, Debug, Default)]
pub struct AIConversation;
