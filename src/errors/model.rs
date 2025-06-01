pub enum ModelError {
    NotFound(String),
    NotSupported(String),
    InvalidInput(String),
    InternalError(String),
}
