pub mod chat_model;
pub mod provider_settings;

use crate::{
    errors::{ModelError, ProviderError},
    provider::LanguageModelProvider,
};
use chat_model::{model_id::OpenAIChatModelId, OpenAIChatModel};
use provider_settings::OpenAIProviderSettings;
use std::str::FromStr;

pub struct OpenAIProvider {
    pub settings: OpenAIProviderSettings,
}

impl OpenAIProvider {
    pub fn new(settings: OpenAIProviderSettings) -> Self {
        OpenAIProvider { settings }
    }

    pub fn default() -> Self {
        OpenAIProvider {
            settings: OpenAIProviderSettings::default(),
        }
    }

    pub fn create_chat_model(
        &self,
        model_id: OpenAIChatModelId,
    ) -> Result<OpenAIChatModel, ModelError> {
        unimplemented!(
            "OpenAIProvider::create_chat_model is not implemented yet. Model ID: {}",
            model_id
        );
    }
}

impl LanguageModelProvider for OpenAIProvider {
    type Model = OpenAIChatModel;
    fn language_model(&self, model_id: &str) -> Result<Self::Model, ProviderError> {
        if model_id.is_empty() {
            return Err(ProviderError::InvalidModelId(format!(
                "Provided an empty OpenAI model id"
            )));
        }
        let model_id = model_id.trim();
        let openai_model_id = OpenAIChatModelId::from_str(model_id).map_err(|_| {
            ProviderError::InvalidModelId(format!(
                "Provided an invalid OpenAI model id, {model_id}"
            ))
        })?;

        self.create_chat_model(openai_model_id)
            .map_err(|e| ProviderError::ModelError(e))
    }

    fn get_headers(&self) -> Result<Vec<(String, String)>, ProviderError> {
        let mut headers = Vec::new();
        headers.push((
            "Authorization".to_string(),
            format!("Bearer {}", self.settings.api_key),
        ));
        headers.push(("Content-Type".to_string(), "application/json".to_string()));
        if let Some(org_id) = &self.settings.organization_id {
            headers.push(("OpenAI-Organization".to_string(), org_id.clone()));
        }
        if let Some(project_id) = &self.settings.project_id {
            headers.push(("OpenAI-Project".to_string(), project_id.clone()));
        }
        Ok(headers)
    }
}
