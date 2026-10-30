pub mod panel {

    use warpui::elements::{Element, Empty};

    use warpui::{AppContext, Entity, View, ViewContext};



    pub fn init(_app: &mut AppContext) {}



    #[derive(Clone, Debug)]

    pub enum AIAssistantPanelEvent {}



    pub struct AIAssistantPanelView;



    impl AIAssistantPanelView {

        pub fn new(

            _server_api: std::sync::Arc<crate::server::server_api::ServerApi>,

            _ai_client: std::sync::Arc<dyn crate::server::server_api::ai::AIClient>,

            _ctx: &mut ViewContext<Self>,

        ) -> Self {

            Self

        }

    }



    impl Entity for AIAssistantPanelView {

        type Event = AIAssistantPanelEvent;

    }



    impl View for AIAssistantPanelView {

        fn ui_name() -> &'static str {

            "AIAssistantPanelViewLite"

        }



        fn render(&self, _app: &AppContext) -> Box<dyn Element> {

            Empty::new().finish()

        }

    }

}



pub mod execution_context {

    pub struct WarpAiExecutionContext;

}



pub mod requests;

pub mod transcript {}

pub mod utils;



pub const AI_ASSISTANT_FEATURE_NAME: &str = "Warp AI";

pub const AI_ASSISTANT_LOGO_COLOR: pathfinder_color::ColorU =

    pathfinder_color::ColorU::new(0, 0, 0, 255);

pub const AI_ASSISTANT_SVG_PATH: &str = "";

pub const ASK_AI_ASSISTANT_TEXT: &str = "Ask Warp AI";



#[derive(Clone, Debug, Default)]

pub enum AskAIType {

    #[default]

    Empty,

}



pub struct AIGeneratedCommand;

#[derive(Clone, Debug)]
pub struct GenerateCommandsFromNaturalLanguageError;



pub use requests::GenerateDialogueResult;

pub use utils::TranscriptPart;
