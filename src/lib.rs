mod errors;
mod model;
mod prompt;
mod provider;
mod utils;

#[cfg(test)]
mod test {
    use crate::provider::{LanguageModelProvider, OpenAIProvider};

    #[test]
    fn test_build() {
        let openai = OpenAIProvider::default();
        let model = match openai.language_model("gpt-4o") {
            Ok(model) => model,
            Err(e) => panic!("Failed to build an openai model: {}", e),
        };
        let response = model
            .generate_text()
            .system("You are a helpful assistant.".into())
            .prompt("What is the capital of France?".into())
            .temperature(0.7)
            .top_p(0.9)
            .top_k(50)
            .presence_penalty(0.1)
            .frequency_penalty(0.1)
            .max_steps(5)
            .max_retries(3)
            .send()
            .await;
    }
}
