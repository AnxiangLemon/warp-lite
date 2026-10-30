use crate::ManagedSecretValue;

#[derive(Debug, thiserror::Error)]
pub enum EnvelopeError {
    #[error("Managed secret encryption is unavailable in this build")]
    Unavailable,
}

pub struct UploadKey(std::convert::Infallible);

impl UploadKey {
    pub fn import_public_keyset(public_key: &str) -> Result<Self, EnvelopeError> {
        let _ = public_key;
        Err(EnvelopeError::Unavailable)
    }

    pub fn encrypt_secret(
        &self,
        actor_uid: &str,
        secret_name: &str,
        secret: &ManagedSecretValue,
    ) -> Result<String, EnvelopeError> {
        let _ = (&self.0, actor_uid, secret_name, secret);
        Err(EnvelopeError::Unavailable)
    }

    pub fn seal_with_context(
        &self,
        context_info: &[u8],
        plaintext: &[u8],
    ) -> Result<String, EnvelopeError> {
        let _ = (&self.0, context_info, plaintext);
        Err(EnvelopeError::Unavailable)
    }
}
