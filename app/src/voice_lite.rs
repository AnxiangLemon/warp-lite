pub mod transcriber {
    use std::sync::Arc;

    use async_trait::async_trait;
    use warpui::{Entity, SingletonEntity};

    use crate::server::server_api::TranscribeError;

    #[async_trait]
    pub trait Transcriber: Send + Sync {
        async fn transcribe(
            &self,
            _wav_base64: String,
            _language: Option<String>,
        ) -> Result<String, TranscribeError> {
            Ok(String::new())
        }
    }

    struct DisabledTranscriber;

    #[async_trait]
    impl Transcriber for DisabledTranscriber {}

    pub struct VoiceTranscriber {
        transcriber: Option<Arc<dyn Transcriber>>,
    }

    impl VoiceTranscriber {
        pub fn new(_transcriber: Arc<dyn Transcriber>) -> Self {
            Self { transcriber: None }
        }

        pub fn disabled() -> Self {
            Self { transcriber: None }
        }

        pub fn transcriber(&self) -> Option<&Arc<dyn Transcriber>> {
            self.transcriber.as_ref()
        }
    }

    impl Entity for VoiceTranscriber {
        type Event = ();
    }

    impl SingletonEntity for VoiceTranscriber {}
}
