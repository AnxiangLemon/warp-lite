use std::collections::{HashMap, HashSet};

use ai::document::{AIDocumentId, AIDocumentVersion};
use chrono::{DateTime, Local};
use warpui::{AppContext, Entity, EntityId, ModelContext, SingletonEntity};

use crate::ai::agent::AIAgentActionId;
use crate::ai::agent::conversation::AIConversationId;

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

pub enum AIDocumentSaveStatus {
    NotSaved,
    Saving,
    Saved,
}

impl AIDocumentSaveStatus {
    pub fn is_saved(&self) -> bool {
        matches!(self, AIDocumentSaveStatus::Saved)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AIDocumentUpdateSource {
    User,
    Agent,
    Restoration,
}

#[derive(Debug, Clone)]
pub struct AIDocumentModel {
    documents: HashMap<AIDocumentId, AIDocument>,
    latest_document_id_by_conversation_id: HashMap<AIConversationId, AIDocumentId>,
    streaming_create_documents:
        HashMap<(AIConversationId, AIAgentActionId, usize), AIDocumentId>,
}

impl AIDocumentModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            documents: HashMap::new(),
            latest_document_id_by_conversation_id: HashMap::new(),
            streaming_create_documents: HashMap::new(),
        }
    }

    #[cfg(any(test, all(feature = "tui", feature = "test-util")))]
    pub fn new_for_test() -> Self {
        Self {
            documents: HashMap::new(),
            latest_document_id_by_conversation_id: HashMap::new(),
            streaming_create_documents: HashMap::new(),
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

    pub fn get_conversation_id_for_document_id(
        &self,
        document_id: &AIDocumentId,
    ) -> Option<AIConversationId> {
        self.documents
            .get(document_id)
            .map(|doc| doc.conversation_id)
    }

    pub fn is_document_visible_by_conversation_in_pane_group(
        &self,
        conversation_id: &AIConversationId,
        _pane_group_id: EntityId,
    ) -> bool {
        self.get_document_id_by_conversation_id(*conversation_id)
            .and_then(|id| self.documents.get(&id))
            .is_some_and(|doc| !doc.visible_in_pane_groups.is_empty())
    }

    pub fn get_document_save_status(&self, _id: &AIDocumentId) -> AIDocumentSaveStatus {
        AIDocumentSaveStatus::NotSaved
    }

    pub fn streaming_document_id_for_create_documents(
        &self,
        conversation_id: &AIConversationId,
        action_id: AIAgentActionId,
        index: usize,
    ) -> Option<AIDocumentId> {
        self.streaming_create_documents
            .get(&(*conversation_id, action_id, index))
            .copied()
    }

    pub fn apply_streamed_agent_update(
        &mut self,
        id: &AIDocumentId,
        title: &str,
        content: &str,
        _ctx: &mut ModelContext<Self>,
    ) {
        if let Some(doc) = self.documents.get_mut(id) {
            doc.title = title.to_owned();
            doc.content = content.to_owned();
        }
    }

    pub fn create_document(
        &mut self,
        title: &str,
        content: String,
        conversation_id: AIConversationId,
        _file_link_resolution_context: Option<()>,
        _ctx: &mut ModelContext<Self>,
    ) -> AIDocumentId {
        let id = AIDocumentId::new();
        self.insert_document(id, title, content, conversation_id);
        id
    }

    pub fn sync_to_warp_drive(&mut self, _id: AIDocumentId, _ctx: &mut ModelContext<Self>) -> bool {
        false
    }

    pub fn clear_streaming_documents_for_action(
        &mut self,
        conversation_id: &AIConversationId,
        action_id: AIAgentActionId,
        _ctx: &mut ModelContext<Self>,
    ) {
        self.streaming_create_documents
            .retain(|(conv, act, _), _| conv != conversation_id || *act != action_id);
    }

    pub fn restore_document(
        &mut self,
        id: AIDocumentId,
        conversation_id: AIConversationId,
        title: String,
        content: String,
        _created_at: DateTime<Local>,
        _ctx: &mut ModelContext<Self>,
    ) {
        self.insert_document(id, &title, content, conversation_id);
    }

    pub fn restore_document_edit(
        &mut self,
        id: &AIDocumentId,
        content: String,
        _created_at: DateTime<Local>,
        _ctx: &mut ModelContext<Self>,
    ) -> Option<AIDocumentVersion> {
        let doc = self.documents.get_mut(id)?;
        doc.content = content;
        doc.version = AIDocumentVersion(doc.version.0 + 1);
        Some(doc.version)
    }

    pub fn delete_document(&mut self, id: &AIDocumentId) {
        if let Some(doc) = self.documents.remove(id) {
            if self
                .latest_document_id_by_conversation_id
                .get(&doc.conversation_id)
                .is_some_and(|latest| latest == id)
            {
                self.latest_document_id_by_conversation_id
                    .remove(&doc.conversation_id);
            }
        }
    }

    fn insert_document(
        &mut self,
        id: AIDocumentId,
        title: &str,
        content: String,
        conversation_id: AIConversationId,
    ) {
        self.documents.insert(
            id,
            AIDocument {
                title: title.to_owned(),
                version: AIDocumentVersion::default(),
                content,
                user_edit_status: AIDocumentUserEditStatus::UpToDate,
                conversation_id,
                visible_in_pane_groups: HashSet::new(),
            },
        );
        self.latest_document_id_by_conversation_id
            .insert(conversation_id, id);
    }
}

pub enum AIDocumentModelEvent {
    DocumentUpdated {
        document_id: AIDocumentId,
        version: AIDocumentVersion,
        source: AIDocumentUpdateSource,
    },
    DocumentSaveStatusUpdated(AIDocumentId),
    DocumentUserEditStatusUpdated {
        document_id: AIDocumentId,
        status: AIDocumentUserEditStatus,
    },
    StreamingDocumentsCleared(AIConversationId),
    DocumentVisibilityChanged(AIDocumentId),
}

impl Entity for AIDocumentModel {
    type Event = AIDocumentModelEvent;
}

impl SingletonEntity for AIDocumentModel {}
