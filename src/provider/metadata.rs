use serde_json::Value as JSONValue;
use std::collections::HashMap;

pub type LanguageModelProviderMetadata = HashMap<String, HashMap<String, JSONValue>>;

