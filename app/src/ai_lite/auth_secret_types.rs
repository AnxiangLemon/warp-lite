pub struct AuthSecret;
pub struct AuthSecretTypeInfo;

pub fn auth_secret_types_for_harness() -> Vec<AuthSecretTypeInfo> {
    vec![]
}

pub fn learn_more_url_for_harness(_harness: warp_cli::agent::Harness) -> Option<String> {
    None
}

pub fn build_managed_secret_value(_secret: &AuthSecret) -> String {
    String::new()
}
