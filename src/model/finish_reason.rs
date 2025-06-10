pub enum LanguageModelFinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    Error,
    Other,
    Unknown,
}

impl Default for LanguageModelFinishReason {
    fn default() -> Self {
        Self::Unknown
    }
}
