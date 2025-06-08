use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Model not found")]
    NotFound(String),
    #[error("Model not supported")]
    NotSupported(String),
    #[error("Invalid Argument Supplued: {0}")]
    InvalidArgument(String),
    #[error("Some Internal error occurred: {0}")]
    InternalError(String),
    #[error("Invalid Prompt Provided: {0}")]
    InvalidPrompt(String),
    #[error("Some Unknown Error Occured: {0}")]
    Other(String),
}
