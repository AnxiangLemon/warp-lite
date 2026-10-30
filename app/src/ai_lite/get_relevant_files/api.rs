use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetRelevantFiles;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetRelevantFilesRequest;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetRelevantFilesResponse;
