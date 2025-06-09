pub struct LanguageModelLogProbs {
    pub token: String,
    pub logprob: f32,
    top_logprobs: Vec<(String, f32)>,
}

