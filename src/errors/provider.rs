use super::ModelError;

pub enum ProviderError {
    ModelNotFound(String),
    InvalidModelId(String),
    RequestFailed(String),
    UnsupportedModel(String),
    ModelError(ModelError),
    UnknownError,
}
