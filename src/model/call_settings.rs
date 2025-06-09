use crate::prompt::{CallSettings, CoreMessage, Prompt};

pub struct GenerateTextOptions {
    pub call_settings: CallSettings,
    pub prompt: Prompt,
    /**
    Maximum number of sequential LLM calls (steps), e.g. when you use tool calls. Must be at least 1.

    A maximum number is required to prevent infinite loops in the case of misconfigured tools.

    By default, it's set to 1, which means that only a single LLM call is made.
         */
    pub max_steps: u32,
}

impl Default for GenerateTextOptions {
    fn default() -> Self {
        GenerateTextOptions {
            call_settings: CallSettings::default(),
            prompt: Prompt::default(),
            max_steps: 1,
        }
    }
}

impl GenerateTextOptions {
    pub fn new() -> Self {
        GenerateTextOptions::default()
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.call_settings.temperature = temperature;
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.call_settings.top_p = Some(top_p);
        self
    }

    pub fn top_k(mut self, top_k: usize) -> Self {
        self.call_settings.top_k = Some(top_k);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.call_settings.presence_penalty = Some(presence_penalty);
        self
    }
    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.call_settings.frequency_penalty = Some(frequency_penalty);
        self
    }
    pub fn stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.call_settings.stop_sequences = Some(stop_sequences);
        self
    }
    pub fn seed(mut self, seed: u64) -> Self {
        self.call_settings.seed = Some(seed);
        self
    }
    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.call_settings.max_retries = max_retries;
        self
    }
    pub fn headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.call_settings.headers = headers;
        self
    }
    pub fn system(mut self, system: String) -> Self {
        self.prompt.system = system;
        self
    }
    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt.prompt = Some(prompt);
        self
    }
    pub fn messages(mut self, messages: Vec<CoreMessage>) -> Self {
        self.prompt.messages = Some(messages);
        self
    }
    pub fn max_steps(mut self, max_steps: u32) -> Self {
        self.max_steps = max_steps;
        self
    }
}
