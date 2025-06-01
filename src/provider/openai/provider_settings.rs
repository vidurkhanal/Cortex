use crate::utils;

const OPNEAI_DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

/// OpenAI compatibility mode. Should be set to `strict` when using the OpenAI API,
/// and `compatible` when using 3rd party providers. In `compatible` mode, newer
/// information such as streamOptions are not being sent. Defaults to `strict`.
///
pub enum OpenAIProviderSettingsCompatibility {
    STRICT,
    COMPATIBLE,
}

pub struct OpenAIProviderSettings {
    /// Base URL for the OpenAI API calls.
    pub base_url: String,
    /// API key for authenticating requests to the OpenAI API.
    pub api_key: String,
    /// Optional organization ID for the OpenAI API.
    pub organization_id: Option<String>,
    /// Optional project ID for the OpenAI API.
    pub project_id: Option<String>,
    // /// Optional headers to include in the requests.
    pub headers: Option<Vec<(String, String)>>,
    /// Compatibility mode for the OpenAI API.
    pub compatibility: OpenAIProviderSettingsCompatibility,
    /// Provider name. Overrides the `openai` default name for 3rd party providers.
    pub name: String,
}

impl Default for OpenAIProviderSettings {
    fn default() -> Self {
        OpenAIProviderSettings {
            base_url: OPNEAI_DEFAULT_BASE_URL.to_string(),
            api_key: String::new(),
            organization_id: None,
            project_id: None,
            headers: None,
            compatibility: OpenAIProviderSettingsCompatibility::STRICT,
            name: "openai".to_string(),
        }
    }
}

impl OpenAIProviderSettings {
    /// Creates a new instance of `OpenAIProviderSettings` with the provided API key.
    pub fn new(api_key: String) -> Self {
        OpenAIProviderSettings {
            api_key,
            ..Default::default()
        }
    }

    /// Sets the API key for the OpenAI provider settings.
    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = utils::without_trailing_slash(base_url);
        self
    }

    /// Sets the API key for the OpenAI provider settings.
    pub fn organization_id(mut self, organization_id: String) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    /// Sets the project ID for the OpenAI provider settings.
    pub fn project_id(mut self, project_id: String) -> Self {
        self.project_id = Some(project_id);
        self
    }

    /// Sets the headers for the OpenAI provider settings.
    pub fn headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Sets the compatibility mode for the OpenAI provider settings.
    pub fn compatibility(mut self, compatibility: OpenAIProviderSettingsCompatibility) -> Self {
        self.compatibility = compatibility;
        self
    }

    /// Sets the name for the OpenAI provider settings.
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}
