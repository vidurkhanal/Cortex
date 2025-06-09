pub mod call_settings;
pub mod call_warning;
pub mod logprobs;
pub mod request_metadata;
pub mod response_metadata;
pub mod step_result;
pub mod tools;

use crate::errors::{self, ModelError};
pub use call_settings::GenerateTextOptions;

pub enum LanguageModelCall {
    GenerateText(GenerateTextOptions),
    GenerateImage(String, u32, u32),
    GenerateObject(String, serde_json::Value),
}

pub trait LanguageModel {
    fn generate_text(&self, generate_text_call: GenerateTextOptions) -> Result<String, ModelError>;
    fn generate_image(
        &self,
        prompt: &str,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, errors::ModelError>;
    fn generate_object(
        &self,
        prompt: &str,
        schema: &serde_json::Value,
    ) -> Result<serde_json::Value, ModelError>;
    fn do_generate(&self, model_call: LanguageModelCall) -> Result<String, ModelError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LanguageModelUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
