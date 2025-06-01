mod openai;

use crate::errors::ProviderError;
use crate::model::LanguageModel;

pub trait LanguageModelProvider {
    fn language_model(&self, model_id: &str) -> Result<Box<dyn LanguageModel>, ProviderError>;
    fn get_headers(&self) -> Result<Vec<(String, String)>, ProviderError>;
}
