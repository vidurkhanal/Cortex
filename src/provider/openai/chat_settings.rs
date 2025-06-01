use std::fmt;
use std::str::FromStr;

use crate::errors::ModelError;
use crate::model::{GenerateTextCallSettings, LanguageModel};

/// https://platform.openai.com/docs/models
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenAIChatModelId {
    // O1 models
    O1,
    O12024_12_17,
    O1Mini,
    O1Mini2024_09_12,
    O1Preview,
    O1Preview2024_09_12,

    // O3 models
    O3Mini,
    O3Mini2025_01_31,
    O3,
    O32025_04_16,

    // O4 models
    O4Mini,
    O4Mini2025_04_16,

    // GPT-4.1 models
    GPT41,
    GPT412025_04_14,
    GPT41Mini,
    GPT41Mini2025_04_14,
    GPT41Nano,
    GPT41Nano2025_04_14,

    // GPT-4o models
    GPT4o,
    GPT4o2024_05_13,
    GPT4o2024_08_06,
    GPT4o2024_11_20,
    GPT4oAudioPreview,
    GPT4oAudioPreview2024_10_01,
    GPT4oAudioPreview2024_12_17,
    GPT4oSearchPreview,
    GPT4oSearchPreview2025_03_11,
    GPT4oMiniSearchPreview,
    GPT4oMiniSearchPreview2025_03_11,
    GPT4oMini,
    GPT4oMini2024_07_18,
    ChatGPT4oLatest,

    // GPT-4 models
    GPT4Turbo,
    GPT4Turbo2024_04_09,
    GPT4TurboPreview,
    GPT40125Preview,
    GPT41106Preview,
    GPT4,
    GPT40613,

    // GPT-4.5 models
    GPT45Preview,
    GPT45Preview2025_02_27,

    // GPT-3.5 models
    GPT35Turbo0125,
    GPT35Turbo,
    GPT35Turbo1106,

    Custom(String),
}

impl FromStr for OpenAIChatModelId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "o1" => Ok(Self::O1),
            "o1-2024-12-17" => Ok(Self::O12024_12_17),
            "o1-mini" => Ok(Self::O1Mini),
            "o1-mini-2024-09-12" => Ok(Self::O1Mini2024_09_12),
            "o1-preview" => Ok(Self::O1Preview),
            "o1-preview-2024-09-12" => Ok(Self::O1Preview2024_09_12),

            "o3-mini" => Ok(Self::O3Mini),
            "o3-mini-2025-01-31" => Ok(Self::O3Mini2025_01_31),
            "o3" => Ok(Self::O3),
            "o3-2025-04-16" => Ok(Self::O32025_04_16),

            "o4-mini" => Ok(Self::O4Mini),
            "o4-mini-2025-04-16" => Ok(Self::O4Mini2025_04_16),

            "gpt-4.1" => Ok(Self::GPT41),
            "gpt-4.1-2025-04-14" => Ok(Self::GPT412025_04_14),
            "gpt-4.1-mini" => Ok(Self::GPT41Mini),
            "gpt-4.1-mini-2025-04-14" => Ok(Self::GPT41Mini2025_04_14),
            "gpt-4.1-nano" => Ok(Self::GPT41Nano),
            "gpt-4.1-nano-2025-04-14" => Ok(Self::GPT41Nano2025_04_14),

            "gpt-4o" => Ok(Self::GPT4o),
            "gpt-4o-2024-05-13" => Ok(Self::GPT4o2024_05_13),
            "gpt-4o-2024-08-06" => Ok(Self::GPT4o2024_08_06),
            "gpt-4o-2024-11-20" => Ok(Self::GPT4o2024_11_20),
            "gpt-4o-audio-preview" => Ok(Self::GPT4oAudioPreview),
            "gpt-4o-audio-preview-2024-10-01" => Ok(Self::GPT4oAudioPreview2024_10_01),
            "gpt-4o-audio-preview-2024-12-17" => Ok(Self::GPT4oAudioPreview2024_12_17),
            "gpt-4o-search-preview" => Ok(Self::GPT4oSearchPreview),
            "gpt-4o-search-preview-2025-03-11" => Ok(Self::GPT4oSearchPreview2025_03_11),
            "gpt-4o-mini-search-preview" => Ok(Self::GPT4oMiniSearchPreview),
            "gpt-4o-mini-search-preview-2025-03-11" => Ok(Self::GPT4oMiniSearchPreview2025_03_11),
            "gpt-4o-mini" => Ok(Self::GPT4oMini),
            "gpt-4o-mini-2024-07-18" => Ok(Self::GPT4oMini2024_07_18),

            "gpt-4-turbo" => Ok(Self::GPT4Turbo),
            "gpt-4-turbo-2024-04-09" => Ok(Self::GPT4Turbo2024_04_09),
            "gpt-4-turbo-preview" => Ok(Self::GPT4TurboPreview),
            "gpt-4-0125-preview" => Ok(Self::GPT40125Preview),
            "gpt-4-1106-preview" => Ok(Self::GPT41106Preview),

            "gpt-4" => Ok(Self::GPT4),
            "gpt-4-0613" => Ok(Self::GPT40613),

            "gpt-4.5-preview" => Ok(Self::GPT45Preview),
            "gpt-4.5-preview-2025-02-27" => Ok(Self::GPT45Preview2025_02_27),

            "gpt-3.5-turbo-0125" => Ok(Self::GPT35Turbo0125),
            "gpt-3.5-turbo" => Ok(Self::GPT35Turbo),
            "gpt-3.5-turbo-1106" => Ok(Self::GPT35Turbo1106),

            "chatgpt-4o-latest" => Ok(Self::ChatGPT4oLatest),

            // Handle custom model IDs
            _ => Ok(Self::Custom(s.to_string())),
        }
    }
}

impl fmt::Display for OpenAIChatModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::O1 => write!(f, "o1"),
            Self::O12024_12_17 => write!(f, "o1-2024-12-17"),
            Self::O1Mini => write!(f, "o1-mini"),
            Self::O1Mini2024_09_12 => write!(f, "o1-mini-2024-09-12"),
            Self::O1Preview => write!(f, "o1-preview"),
            Self::O1Preview2024_09_12 => write!(f, "o1-preview-2024-09-12"),

            Self::O3Mini => write!(f, "o3-mini"),
            Self::O3Mini2025_01_31 => write!(f, "o3-mini-2025-01-31"),
            Self::O3 => write!(f, "o3"),
            Self::O32025_04_16 => write!(f, "o3-2025-04-16"),

            Self::O4Mini => write!(f, "o4-mini"),
            Self::O4Mini2025_04_16 => write!(f, "o4-mini-2025-04-16"),

            Self::GPT41 => write!(f, "gpt-4.1"),
            Self::GPT412025_04_14 => write!(f, "gpt-4.1-2025-04-14"),
            Self::GPT41Mini => write!(f, "gpt-4.1-mini"),
            Self::GPT41Mini2025_04_14 => write!(f, "gpt-4.1-mini-2025-04-14"),
            Self::GPT41Nano => write!(f, "gpt-4.1-nano"),
            Self::GPT41Nano2025_04_14 => write!(f, "gpt-4.1-nano-2025-04-14"),

            Self::GPT4o => write!(f, "gpt-4o"),
            Self::GPT4o2024_05_13 => write!(f, "gpt-4o-2024-05-13"),
            Self::GPT4o2024_08_06 => write!(f, "gpt-4o-2024-08-06"),
            Self::GPT4o2024_11_20 => write!(f, "gpt-4o-2024-11-20"),
            Self::GPT4oAudioPreview => write!(f, "gpt-4o-audio-preview"),
            Self::GPT4oAudioPreview2024_10_01 => write!(f, "gpt-4o-audio-preview-2024-10-01"),
            Self::GPT4oAudioPreview2024_12_17 => write!(f, "gpt-4o-audio-preview-2024-12-17"),
            Self::GPT4oSearchPreview => write!(f, "gpt-4o-search-preview"),
            Self::GPT4oSearchPreview2025_03_11 => write!(f, "gpt-4o-search-preview-2025-03-11"),
            Self::GPT4oMiniSearchPreview => write!(f, "gpt-4o-mini-search-preview"),
            Self::GPT4oMiniSearchPreview2025_03_11 => {
                write!(f, "gpt-4o-mini-search-preview-2025-03-11")
            }
            Self::GPT4oMini => write!(f, "gpt-4o-mini"),
            Self::GPT4oMini2024_07_18 => write!(f, "gpt-4o-mini-2024-07-18"),

            Self::GPT4Turbo => write!(f, "gpt-4-turbo"),
            Self::GPT4Turbo2024_04_09 => write!(f, "gpt-4-turbo-2024-04-09"),
            Self::GPT4TurboPreview => write!(f, "gpt-4-turbo-preview"),
            Self::GPT40125Preview => write!(f, "gpt-4-0125-preview"),
            Self::GPT41106Preview => write!(f, "gpt-4-1106-preview"),

            Self::GPT4 => write!(f, "gpt-4"),
            Self::GPT40613 => write!(f, "gpt-4-0613"),

            Self::GPT45Preview => write!(f, "gpt-4.5-preview"),
            Self::GPT45Preview2025_02_27 => write!(f, "gpt-4.5-preview-2025-02-27"),

            Self::GPT35Turbo0125 => write!(f, "gpt-3.5-turbo-0125"),
            Self::GPT35Turbo => write!(f, "gpt-3.5-turbo"),
            Self::GPT35Turbo1106 => write!(f, "gpt-3.5-turbo-1106"),

            Self::ChatGPT4oLatest => write!(f, "chatgpt-4o-latest"),

            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

pub enum OpenAIChatSettingsReasoningEffort {
    /// No reasoning effort, the model will not perform any reasoning.
    None,

    /// Low reasoning effort, the model will perform minimal reasoning.
    Low,

    /// Medium reasoning effort, the model will perform moderate reasoning.
    Medium,

    /// High reasoning effort, the model will perform extensive reasoning.
    High,
}

pub struct OpenAIChatModel {
    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a JSON object that maps tokens (specified by their token ID in
    /// the GPT tokenizer) to an associated bias value from -100 to 100. You
    /// the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 shoul
    /// }
    ///
    /// decrease or increase likelihood of selection; values like -100 or 100
    /// should result in a ban or exclusive selection of the relevant token.
    ///
    /// As an example, you can pass {"50256": -100} to prevent the <|endoftext|>
    /// token from being generated.
    pub logit_bias: Option<Vec<(String, f32)>>,
    /// Return the log probabilities of the tokens. Including logprobs will increase
    /// the response size and can slow down response times. However, it can
    /// be useful to better understand how the model is behaving.
    ///
    /// Setting to true will return the log probabilities of the tokens that
    /// were generated.
    ///
    /// Setting to a number will return the log probabilities of the top n
    /// tokens that were generated.
    pub log_probs: Option<u32>,
    /// Whether to enable parallel function calling during tool use. Default to true.
    pub parallel_calls: bool,
    /// Whether to use structured outputs. Defaults to false.
    ///
    /// When enabled, tool calls and object generation will be strict and follow the provided schema.
    pub structured_output: bool,
    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse. Learn more.
    pub user: Option<String>,
    /// Automatically download images and pass the image as data to the model.
    /// OpenAI supports image URLs for public models, so this is only needed for
    /// private models or when the images are not publicly accessible.
    ///
    /// Defaults to `false`.
    pub download_images: bool,
    /// Reasoning effort for reasoning models. Defaults to `medium`.
    pub reasoning_effort: OpenAIChatSettingsReasoningEffort,
}

impl Default for OpenAIChatModel {
    fn default() -> Self {
        OpenAIChatModel {
            logit_bias: None,
            log_probs: None,
            parallel_calls: true,
            structured_output: false,
            user: None,
            download_images: false,
            reasoning_effort: OpenAIChatSettingsReasoningEffort::Medium,
        }
    }
}

impl OpenAIChatModel {
    pub fn new() -> Self {
        OpenAIChatModel::default()
    }

    pub fn generate() -> Result<(), ModelError> {
        Err(ModelError::NotSupported(
            "OpenAIChatModel does not support generate operation".to_string(),
        ))
    }

    pub fn with_logit_bias(mut self, logit_bias: Vec<(String, f32)>) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn with_log_probs(mut self, log_probs: u32) -> Self {
        self.log_probs = Some(log_probs);
        self
    }

    pub fn with_parallel_calls(mut self, parallel_calls: bool) -> Self {
        self.parallel_calls = parallel_calls;
        self
    }

    pub fn with_structured_output(mut self, structured_output: bool) -> Self {
        self.structured_output = structured_output;
        self
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_download_images(mut self, download_images: bool) -> Self {
        self.download_images = download_images;
        self
    }

    pub fn with_reasoning_effort(
        mut self,
        reasoning_effort: OpenAIChatSettingsReasoningEffort,
    ) -> Self {
        self.reasoning_effort = reasoning_effort;
        self
    }
}

impl LanguageModel for OpenAIChatModel {
    fn generate_text(&self) -> Result<GenerateTextCallSettings, crate::errors::ModelError> {
        todo!()
    }

    fn generate_image(
        &self,
        prompt: &str,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, crate::errors::ModelError> {
        todo!()
    }

    fn generate_object(
        &self,
        prompt: &str,
        schema: &serde_json::Value,
    ) -> Result<serde_json::Value, ModelError> {
        todo!()
    }
}
