pub mod call_settings;
pub mod call_warning;
pub mod finish_reason;
pub mod function_tool_call;
pub mod logprobs;
pub mod message;
pub mod request_metadata;
pub mod response_metadata;
pub mod source;
pub mod step_result;
pub mod tools;
pub mod usage;

use crate::{
    core::generate_text::GenerateTextOptions, errors::ModelError,
    provider::metadata::LanguageModelProviderMetadata,
};
use call_settings::LanguageModelCallSettings;
use call_warning::LanguageModelCallWarning;
use finish_reason::LanguageModelFinishReason;
use function_tool_call::LanguageModelFunctionToolCall;
use logprobs::LanguageModelLogprobs;
use message::LanguageModelMessage;
use request_metadata::LanguageModelRequestMetadata;
use response_metadata::LanguageModelResponseMetadata;
use source::LanguageModelSource;
use usage::LanguageModelUsage;

pub enum LanguageModelCall {
    GenerateText(GenerateTextOptions),
    GenerateImage(String, u32, u32),
    GenerateObject(String, serde_json::Value),
}

pub trait LanguageModel {
    fn supports_urls(&self, url: String) -> bool;
    fn do_generate(
        &self,
        request: LanguageModelDoGenerateRequest,
    ) -> Result<LanguageModelDoGenerateResponse, ModelError>;
}

pub struct LanguageModelDoGenerateRequest {
    call_settings: Option<LanguageModelCallSettings>,
    input_format: LanguageModelDoGenerateRequestInputFormat,
    prompt: Vec<LanguageModelMessage>,
    provider_metadata: Option<LanguageModelProviderMetadata>,
}

pub enum LanguageModelDoGenerateRequestInputFormat {
    Messages,
    Prompt,
}

#[derive(Default)]
pub struct LanguageModelDoGenerateResponse {
    text: Option<String>,
    reasoning: Vec<LanguageModelDoGenerateResponseReasoning>,
    files: Vec<LanguageModelDoGenerateResponseFiles>,
    tool_calls: Vec<LanguageModelFunctionToolCall>,
    finish_reason: LanguageModelFinishReason,
    usage: LanguageModelUsage,
    request_body: Option<LanguageModelRequestMetadata>,
    response: Option<LanguageModelResponseMetadata>,
    warnings: Vec<LanguageModelCallWarning>,
    provider_metadata: Option<LanguageModelProviderMetadata>,
    sources: Option<LanguageModelSource>,
    logprobs: Option<LanguageModelLogprobs>,
}

pub enum LanguageModelDoGenerateResponseReasoning {
    Text {
        text: String,
        signature: Option<String>,
    },
    Redacted(String),
}

pub struct LanguageModelDoGenerateResponseFiles {
    pub file_content: LanguageModelDoGenerateResponseFilesContent,
    pub mime_type: String,
}

pub enum LanguageModelDoGenerateResponseFilesContent {
    Base64(String),
    Buffer(Vec<u8>),
}
