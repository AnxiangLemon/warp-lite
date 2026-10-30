use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ModelConfigError {
    pub file_name: String,
    pub file_path: PathBuf,
    pub error_message: String,
}

pub struct CustomModelRouter;

pub fn parse_model_config_yaml(
    _yaml: &str,
    _path: Option<PathBuf>,
) -> Result<CustomModelRouter, ModelConfigError> {
    Err(ModelConfigError {
        file_name: String::new(),
        file_path: PathBuf::new(),
        error_message: "unavailable in warp_lite".to_owned(),
    })
}

pub fn is_custom_router_id(_id: &str) -> bool {
    false
}
