pub mod metadata;

use crate::errors::ProviderError;
use crate::model::LanguageModel;

pub trait LanguageModelProvider {
    type Model: LanguageModel;
    fn language_model(&self, model_id: &str) -> Result<Self::Model, ProviderError>;
    fn get_headers(&self) -> Result<Vec<(String, String)>, ProviderError>;
}
