use std::collections::{HashMap, HashSet};

use ai::document::{AIDocumentId, AIDocumentVersion};
use uuid::Uuid;
use warpui::{AppContext, Entity, EntityId, ModelContext, SingletonEntity};

use crate::ai::agent::conversation::AIConversationId;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AIAgentActionId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIDocumentUserEditStatus {
    UpToDate,
    Dirty,
}

impl AIDocumentUserEditStatus {
    pub fn is_dirty(&self) -> bool {
        matches!(self, AIDocumentUserEditStatus::Dirty)
    }
}

#[derive(Debug, Clone)]
pub struct AIDocument {
    pub title: String,
    pub version: AIDocumentVersion,
    pub content: String,
    pub user_edit_status: AIDocumentUserEditStatus,
    pub conversation_id: AIConversationId,
    pub visible_in_pane_groups: HashSet<EntityId>,
}

#[derive(Debug, Clone)]
pub struct AIDocumentModel {
    documents: HashMap<AIDocumentId, AIDocument>,
    latest_document_id_by_conversation_id: HashMap<AIConversationId, AIDocumentId>,
}

impl AIDocumentModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            documents: HashMap::new(),
            latest_document_id_by_conversation_id: HashMap::new(),
        }
    }

    pub fn get_current_document(&self, id: &AIDocumentId) -> Option<AIDocument> {
        self.documents.get(id).cloned()
    }

    pub fn get_document_content(&self, id: &AIDocumentId, _ctx: &AppContext) -> Option<String> {
        self.documents.get(id).map(|doc| doc.content.clone())
    }

    pub fn get_all_documents_for_conversation(
        &self,
        conversation_id: AIConversationId,
    ) -> Vec<&AIDocument> {
        self.documents
            .values()
            .filter(|doc| doc.conversation_id == conversation_id)
            .collect()
    }

    pub fn get_document_id_by_conversation_id(
        &self,
        conversation_id: AIConversationId,
    ) -> Option<AIDocumentId> {
        self.latest_document_id_by_conversation_id
            .get(&conversation_id)
            .copied()
    }
}

impl Entity for AIDocumentModel {
    type Event = ();
}

impl SingletonEntity for AIDocumentModel {}
