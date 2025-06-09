use super::tools::Tool;

pub enum LanguageModelCallWarning {
    UnsupportedSetting {
        setting: String,
        details: Option<String>,
    },
    UnsupportedTool {
        tool: Tool,
        details: Option<String>,
    },
    Other {
        message: String,
    },
}

