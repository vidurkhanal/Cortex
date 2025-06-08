mod call_settings;
mod content_part;
mod message;
pub mod standarize_prompt;

pub use call_settings::CallSettings;
pub use message::*;

/// Prompt part of the AI function options.
/// It contains a system message, a simple text prompt, or a list of messages.
#[derive(Default)]
pub struct Prompt {
    /// System message to include in the prompt. Can be used with `prompt` or `messages`.
    pub system: String,

    /// A simple text prompt. You can either use `prompt` or `messages` but not both.
    pub prompt: Option<String>,

    /// A list of messages. You can either use `prompt` or `messages` but not both.
    pub messages: Option<Vec<CoreMessage>>,
}
