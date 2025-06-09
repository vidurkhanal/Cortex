mod errors;
mod generate_file;
mod model;
mod prompt;
mod provider;
mod providers;
mod utils;

#[cfg(test)]
mod test {
    use crate::{
        model::{GenerateTextOptions, LanguageModel},
        provider::LanguageModelProvider,
        providers::openai::OpenAIProvider,
    };

    #[test]
    fn test_build() {
        let openai = OpenAIProvider::default();
        let model = match openai.language_model("gpt-4o") {
            Ok(model) => model,
            Err(e) => panic!("Failed to build an openai model: {}", e),
        };
        let response = model.generate_text(
            GenerateTextOptions::default()
                .system("You are a helpful assistant.".into())
                .prompt("What is the capital of Nepal?".into()),
        );
        match response {
            Ok(text) => println!("Response: {}", text),
            Err(e) => panic!("Failed to generate text: {}", e),
        }
    }
}
