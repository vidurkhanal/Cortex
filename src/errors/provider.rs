use super::ModelError;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("model not found: {0}")]
    ModelNotFound(String),
    #[error("invalid model ID: {0}")]
    InvalidModelId(String),
    #[error("invalid provider configuration: {0}")]
    RequestFailed(String),
    #[error("unsupported model: {0}")]
    UnsupportedModel(String),
    #[error("invalid request: {0}")]
    ModelError(ModelError),
    #[error("Unknown error occurred")]
    UnknownError,
}
