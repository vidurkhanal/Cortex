mod call_settings;
mod tools;

use crate::errors::{self, ModelError};
pub use call_settings::GenerateTextCallSettings;
pub use tools::Tools;

pub trait LanguageModel {
    fn generate_text(&self) -> Result<GenerateTextCallSettings, errors::ModelError>;
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
}
