#[derive(Debug, PartialEq)]
pub struct CallSettings {
    /// Maximum number of tokens to generate.
    ///
    /// Default: 2056
    pub max_tokens: usize,

    /// Temperature setting. This is a number between 0 (almost no randomness) and
    /// 1 (very random).
    ///
    /// It is recommended to set either `temperature` or `top_p`, but not both.
    ///
    /// Default: 0
    pub temperature: f32,

    /// Nucleus sampling. This is a number between 0 and 1.
    ///
    /// E.g. 0.1 would mean that only tokens with the top 10% probability mass
    /// are considered.
    ///
    /// It is recommended to set either `temperature` or `top_p`, but not both.
    pub top_p: Option<f32>,

    /// Only sample from the top K options for each subsequent token.
    ///
    /// Used to remove "long tail" low probability responses.
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    pub top_k: Option<usize>,

    /// Presence penalty setting. It affects the likelihood of the model to
    /// repeat information that is already in the prompt.
    ///
    /// The presence penalty is a number between -1 (increase repetition)
    /// and 1 (maximum penalty, decrease repetition). 0 means no penalty.
    pub presence_penalty: Option<f32>,

    /// Frequency penalty setting. It affects the likelihood of the model
    /// to repeatedly use the same words or phrases.
    ///
    /// The frequency penalty is a number between -1 (increase repetition)
    /// and 1 (maximum penalty, decrease repetition). 0 means no penalty.
    pub frequency_penalty: Option<f32>,

    /// Stop sequences.
    /// If set, the model will stop generating text when one of the stop sequences is generated.
    /// Providers may have limits on the number of stop sequences.
    pub stop_sequences: Option<Vec<String>>,

    /// The seed (integer) to use for random sampling. If set and supported
    /// by the model, calls will generate deterministic results.
    pub seed: Option<u64>,

    /// Maximum number of retries. Set to 0 to disable retries.
    ///
    /// Default: 2
    pub max_retries: usize,

    /// Additional HTTP headers to be sent with the request.
    /// Only applicable for HTTP-based providers.
    pub headers: Vec<(String, String)>,
}

impl Default for CallSettings {
    fn default() -> Self {
        CallSettings {
            max_tokens: 2056,
            temperature: 0,
            top_p: None,
            top_k: None,
            presence_penalty: None,
            frequency_penalty: None,
            stop_sequences: None,
            seed: None,
            max_retries: 2,
            headers: Vec::new(),
        }
    }
}
