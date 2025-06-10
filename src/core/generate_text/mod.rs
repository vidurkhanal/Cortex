pub mod options;

use crate::{
    errors::ModelError,
    model::{usage::LanguageModelUsage, LanguageModel, LanguageModelDoGenerateResponse},
    prompt::{standarize_prompt::StandardizedPrompt, RetryPolicy},
};
pub use options::GenerateTextOptions;

pub fn generate_text<T: LanguageModel>(
    model: &mut T,
    options: GenerateTextOptions,
) -> Result<String, ModelError> {
    if options.max_steps < 1 {
        return Err(ModelError::InvalidArgument(format!(
            "OpenAIChatModel requires at least 1 step, got {}",
            options.max_steps
        )));
    }

    let retry_policy = RetryPolicy::new(options.call_settings.max_retries);
    let initial_prompt = StandardizedPrompt::try_from(options.prompt)?;
    let current_model_response = LanguageModelDoGenerateResponse::default();
    let current_tool_calls = Vec::new();
    let current_mode_usage = LanguageModelUsage::default();

    Ok("yeah yeah yeha".to_string())
}
