pub mod prompt_suggestions {
    pub static ACCEPT_PROMPT_SUGGESTION_KEYBINDING: &str = "";

    pub fn has_pending_code_or_unit_test_prompt_suggestion(_app: &warpui::AppContext) -> bool {
        false
    }

    pub fn is_accept_prompt_suggestion_bound_to_cmd_enter(_app: &warpui::AppContext) -> bool {
        false
    }

    pub fn is_accept_prompt_suggestion_bound_to_ctrl_enter(_app: &warpui::AppContext) -> bool {
        false
    }
}

pub mod generate_ai_input_suggestions {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct GenerateAIInputSuggestionsRequest;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct GenerateAIInputSuggestionsResponseV1;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct GenerateAIInputSuggestionsResponseV2;

    pub fn generate(_app: &warpui::AppContext) {}
}

pub mod generate_am_query_suggestions {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct GenerateAMQuerySuggestionsRequest;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct GenerateAMQuerySuggestionsResponse;

    pub fn generate(_app: &warpui::AppContext) {}
}

pub mod predict_am_queries {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct PredictAMQueriesRequest;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct PredictAMQueriesResponse;

    pub fn predict(_app: &warpui::AppContext) {}
}

pub mod next_command_model {
    use warpui::{Entity, ModelContext, SingletonEntity};

    pub struct NextCommandModel;
    pub struct HistoryBasedAutosuggestionState;
    pub struct NextCommandSuggestionState;
    pub struct ZeroStateSuggestionInfo;

    #[derive(Clone, Debug)]
    pub enum NextCommandModelEvent {}

    impl NextCommandModel {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn get_state(&self) -> NextCommandSuggestionState {
            NextCommandSuggestionState
        }
    }

    impl Entity for NextCommandModel {
        type Event = NextCommandModelEvent;
    }

    impl SingletonEntity for NextCommandModel {}

    pub fn is_command_valid(_command: &str) -> bool {
        false
    }

    pub fn is_next_command_enabled(_app: &warpui::AppContext) -> bool {
        false
    }
}
