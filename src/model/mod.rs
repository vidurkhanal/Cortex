mod call_settings;
mod tools;

use crate::errors::{self, ModelError};
pub use call_settings::GenerateTextCallSettings;

pub enum LanguageModelCall {
    GenerateText(GenerateTextCallSettings),
    GenerateImage(String, u32, u32),
    GenerateObject(String, serde_json::Value),
}

pub trait LanguageModel {
    fn generate_text(
        &self,
        generate_text_call: GenerateTextCallSettings,
    ) -> Result<String, ModelError>;
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
