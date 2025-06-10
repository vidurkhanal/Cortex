use std::any;

use crate::provider::metadata::LanguageModelProviderMetadata;

pub enum LanguageModelMessage {
    System(String),
    User(LanguageModelUserMessage),
    Assistant(LanguageModelAssistantMessage),
    Tool(Vec<LanguageModelToolResultPart>),
}

pub enum LanguageModelUserMessage {
    Text(LanguageModelTextPart),
    Image(LanguageModelImagePart),
    File(LanguageModelFilePart),
}

pub enum LanguageModelAssistantMessage {
    Text(LanguageModelTextPart),
    Image(LanguageModelImagePart),
    File(LanguageModelFilePart),
    Reasoning(LanguageModelReasoningPart),
    RedactedReasoning(LanguageModelRedactedReasoningPart),
    ToolCall(LanguageModelToolCallPart),
}

pub struct LanguageModelToolResultPart {
    pub tool_call_id: String,
    pub tool_name: String,
    pub result: any,
    pub is_error: Option<bool>,
    pub content: Vec<LanguageModelToolResultPartContent>,
    pub provider_metadata: Option<LanguageModelProviderMetadata>,
}

pub enum LanguageModelToolResultPartContent {
    Text(String),
    /// Image URL and optional MIME type
    Image(String, Option<String>),
}

pub struct LanguageModelReasoningPart {
    pub text: String,
    pub signature: Option<String>,
    pub provider_metadata: Option<LanguageModelProviderMetadata>,
}

pub struct LanguageModelRedactedReasoningPart {
    pub data: String,
    pub provider_metadata: Option<LanguageModelProviderMetadata>,
}

pub struct LanguageModelToolCallPart {
    pub tool_call_id: String,
    pub tool_name: String,
    pub args: Vec<any>,
    pub provider_metadata: Option<LanguageModelProviderMetadata>,
}

pub struct LanguageModelTextPart {
    pub text: String,
    pub provider_metadata: Option<LanguageModelProviderMetadata>,
}

pub struct LanguageModelImagePart {
    pub image: LanguageModelImagePartContent,
    pub mime_type: Option<String>,
    pub provider_metadata: Option<LanguageModelProviderMetadata>,
}

pub struct LanguageModelFilePart {
    pub file_content: LanguageModelFilePartContent,
    pub mime_type: Option<String>,
    pub provider_metadata: Option<LanguageModelProviderMetadata>,
}

pub enum LanguageModelImagePartContent {
    Base64(String),
    Url(String),
    Buffer(Vec<u8>),
}

pub enum LanguageModelFilePartContent {
    Base64(String),
    Url(String),
}
