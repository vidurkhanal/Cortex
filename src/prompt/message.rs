use super::content_part::{AssistantContent, ToolResultPart, UserContent};

pub struct CoreSystemMessage {
    pub content: String,
}

pub struct CoreUserMessage {
    pub content: UserContent,
}

pub struct CoreAssistantMessage {
    pub content: AssistantContent,
}

pub struct CoreToolMessage {
    pub content: Vec<ToolResultPart>,
}

pub enum CoreMessage {
    System(CoreSystemMessage),
    User(CoreUserMessage),
    Assistant(CoreAssistantMessage),
    Tool(CoreToolMessage),
}
