use crate::provider::metadata::LanguageModelProviderMetadata;

pub enum LanguageModelSourceType {
    Url,
}

pub struct LanguageModelSource {
    source_type: LanguageModelSourceType,
    id: String,
    url: String,
    title: Option<String>,
    provider_metadata: LanguageModelProviderMetadata,
}
