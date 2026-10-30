use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TranscribeRequest;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TranscribeResponse;

pub fn transcribe(_audio: &[u8]) -> Option<String> {
    None
}
