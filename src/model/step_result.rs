use crate::{prompt::{CoreAssistantMessage, CoreToolMessage}, provider::metadata::LanguageModelProviderMetadata};
use super::{call_warning::LanguageModelCallWarning, finish_reason::LanguageModelFinishReason, logprobs::LanguageModelLogprobs, request_metadata::LanguageModelRequestMetadata, response_metadata::LanguageModelResponseMetadata, source::LanguageModelSource, tools::Tool, usage::LanguageModelUsage};


pub enum ResponseMessage {
    AssistantResponse(String, CoreAssistantMessage),
    ToolResponse(String, CoreToolMessage),
}

pub struct StepResultResponse{
    model_response: LanguageModelResponseMetadata,
    messages: Vec<ResponseMessage>,
    body: String,
}

pub enum StepType{
    Initial,
    Continue,
    ToolResult,
}

pub struct StepResult{
    text: String,
    // INFO: this maps to ai sdk's reasoning
    reasoning_text: String,
    // INFO: this maps to ai sdk's reasoningDetails
    reasoning: String,
    files: Vec<GeneratedFile>,
    sources: Vec<LanguageModelSource>,
    tool_calls: Vec<Tool>,
    tool_results: Vec<ToolCallResult>
    finish_reason: LanguageModelFinishReason,
    usage: LanguageModelUsage,
    warnings: Option<Vec<LanguageModelCallWarning>>,
    logprobs: Option<LanguageModelLogprobs>,
    request: LanguageModelRequestMetadata,
    response: StepResultResponse,
    provider_metadata: Option<LanguageModelProviderMetadata>,
    step_type: StepType,
    is_continued: bool,
}
