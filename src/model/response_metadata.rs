pub struct LanguageModelResponseMetadata {
    pub id: String,
    pub timestamp: u64,
    pub model_id: String,
    pub headers: Vec<(String, String)>,
}
