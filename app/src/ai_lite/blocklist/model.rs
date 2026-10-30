use std::marker::PhantomData;

use anyhow::Result;
use chrono::TimeDelta;
use session_sharing_protocol::common::ParticipantId;
use warpui::{AppContext, Entity, ViewContext};

use crate::ai::agent::conversation::{AIConversation, AIConversationId};
use crate::ai::agent::{
    AIAgentAction, AIAgentActionId, AIAgentExchangeId, AIAgentInput, AIAgentOutput,
    CancellationReason, PassiveSuggestionTrigger, RenderableAIError, ServerOutputId, Shared,
};
use crate::ai::blocklist::{BlocklistAIActionModel, BlocklistAIHistoryModel};
use crate::ai::llms::LLMId;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PassiveRequestType {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AIRequestType {
    #[default]
    Active,
    Passive(PassiveRequestType),
}

impl AIRequestType {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_passive(&self) -> bool {
        matches!(self, Self::Passive(_))
    }
}

#[derive(Clone, Debug)]
pub enum AIBlockOutputStatus {
    Pending,
    PartiallyReceived {
        output: Shared<AIAgentOutput>,
    },
    Complete {
        output: Shared<AIAgentOutput>,
    },
    Cancelled {
        partial_output: Option<Shared<AIAgentOutput>>,
        reason: CancellationReason,
    },
    Failed {
        partial_output: Option<Shared<AIAgentOutput>>,
        error: RenderableAIError,
    },
}

impl AIBlockOutputStatus {
    pub fn is_streaming(&self) -> bool {
        matches!(
            self,
            Self::Pending | Self::PartiallyReceived { .. }
        )
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled { .. })
    }

    pub fn cancellation_reason(&self) -> Option<&CancellationReason> {
        match self {
            Self::Cancelled { reason, .. } => Some(reason),
            _ => None,
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete { .. })
    }

    pub fn output_to_render(&self) -> Option<Shared<AIAgentOutput>> {
        match self {
            Self::Pending => None,
            Self::PartiallyReceived { output } => Some(output.get_owned()),
            Self::Complete { output } => Some(output.get_owned()),
            Self::Cancelled { partial_output, .. } => {
                partial_output.as_ref().map(Shared::get_owned)
            }
            Self::Failed { partial_output, .. } => {
                partial_output.as_ref().map(Shared::get_owned)
            }
        }
    }

    pub fn error(&self) -> Option<&RenderableAIError> {
        match self {
            Self::Failed { error, .. } => Some(error),
            _ => None,
        }
    }
}

pub type OutputStatusUpdateCallback<V> = Box<dyn FnMut(&mut V, &mut ViewContext<V>)>;

pub trait AIBlockModel {
    type View;

    fn status(&self, app: &AppContext) -> AIBlockOutputStatus;
    fn server_output_id(&self, app: &AppContext) -> Option<ServerOutputId>;
    fn model_id(&self, app: &AppContext) -> Option<LLMId>;
    fn is_restored(&self) -> bool {
        false
    }
    fn is_forked(&self) -> bool {
        false
    }
    fn was_autodetected_ai_query(&self, _app: &AppContext) -> bool {
        false
    }
    fn time_since_request_start(&self, _app: &AppContext) -> Option<TimeDelta> {
        None
    }
    fn base_model<'a>(&'a self, _app: &'a AppContext) -> Option<&'a LLMId> {
        None
    }
    fn inputs_to_render<'a>(&'a self, _app: &'a AppContext) -> &'a [AIAgentInput];
    fn conversation_id(&self, _app: &AppContext) -> Option<AIConversationId>;
    fn exchange_id(&self, _app: &AppContext) -> Option<AIAgentExchangeId> {
        None
    }
    fn response_initiator(&self, _app: &AppContext) -> Option<ParticipantId> {
        None
    }
    fn on_updated_output(
        &self,
        _callback: OutputStatusUpdateCallback<Self::View>,
        _ctx: &mut ViewContext<Self::View>,
    ) {
    }
    fn request_type(&self, _app: &AppContext) -> AIRequestType {
        AIRequestType::Active
    }
}

pub trait AIBlockModelHelper {
    fn is_first_action_in_output(&self, action_id: &AIAgentActionId, app: &AppContext) -> bool;
    fn conversation<'a>(&self, app: &'a AppContext) -> Option<&'a AIConversation>;
    fn contains_static_prompt_suggestion_input(&self, app: &AppContext) -> bool;
    fn contains_create_document_action(&self, app: &AppContext) -> bool;
    fn contains_update_document_action(&self, app: &AppContext) -> bool;
    fn is_latest_visible_exchange_in_root_task(&self, app: &AppContext) -> bool;
    fn is_latest_exchange_in_terminal_pane(
        &self,
        terminal_view_id: warpui::EntityId,
        app: &AppContext,
    ) -> bool;
    fn is_conversation_summarization_active(&self, app: &AppContext) -> bool;
    fn blocked_action(
        &self,
        action_model: &warpui::ModelHandle<BlocklistAIActionModel>,
        app: &AppContext,
    ) -> Option<AIAgentAction>;
}

pub struct AIBlockModelImpl<V> {
    exchange_id: AIAgentExchangeId,
    conversation_id: AIConversationId,
    is_restored: bool,
    is_forked: bool,
    _view: PhantomData<V>,
}

impl<V> AIBlockModelImpl<V>
where
    V: Entity,
{
    pub fn new(
        exchange_id: AIAgentExchangeId,
        conversation_id: AIConversationId,
        is_restored: bool,
        is_forked: bool,
        _app: &AppContext,
    ) -> Result<Self> {
        Ok(Self {
            exchange_id,
            conversation_id,
            is_restored,
            is_forked,
            _view: PhantomData,
        })
    }
}

impl<V> AIBlockModel for AIBlockModelImpl<V>
where
    V: Entity,
{
    type View = V;

    fn status(&self, _app: &AppContext) -> AIBlockOutputStatus {
        AIBlockOutputStatus::Pending
    }

    fn server_output_id(&self, _app: &AppContext) -> Option<ServerOutputId> {
        None
    }

    fn model_id(&self, _app: &AppContext) -> Option<LLMId> {
        None
    }

    fn is_restored(&self) -> bool {
        self.is_restored
    }

    fn is_forked(&self) -> bool {
        self.is_forked
    }

    fn inputs_to_render<'a>(&'a self, _app: &'a AppContext) -> &'a [AIAgentInput] {
        &[]
    }

    fn conversation_id(&self, _app: &AppContext) -> Option<AIConversationId> {
        Some(self.conversation_id)
    }

    fn exchange_id(&self, _app: &AppContext) -> Option<AIAgentExchangeId> {
        Some(self.exchange_id)
    }
}

impl<V> AIBlockModelHelper for AIBlockModelImpl<V>
where
    V: Entity,
{
    fn is_first_action_in_output(&self, _action_id: &AIAgentActionId, _app: &AppContext) -> bool {
        false
    }

    fn conversation<'a>(&self, app: &'a AppContext) -> Option<&'a AIConversation> {
        self.conversation_id(app)
            .and_then(|id| BlocklistAIHistoryModel::as_ref(app).conversation(&id))
    }

    fn contains_static_prompt_suggestion_input(&self, _app: &AppContext) -> bool {
        false
    }

    fn contains_create_document_action(&self, _app: &AppContext) -> bool {
        false
    }

    fn contains_update_document_action(&self, _app: &AppContext) -> bool {
        false
    }

    fn is_latest_visible_exchange_in_root_task(&self, _app: &AppContext) -> bool {
        false
    }

    fn is_latest_exchange_in_terminal_pane(
        &self,
        _terminal_view_id: warpui::EntityId,
        _app: &AppContext,
    ) -> bool {
        false
    }

    fn is_conversation_summarization_active(&self, _app: &AppContext) -> bool {
        false
    }

    fn blocked_action(
        &self,
        _action_model: &warpui::ModelHandle<BlocklistAIActionModel>,
        _app: &AppContext,
    ) -> Option<AIAgentAction> {
        None
    }
}

pub struct FakeAIBlockModel;
