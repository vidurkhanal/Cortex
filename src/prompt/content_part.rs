pub struct TextPart {
    pub text: String,
}

pub struct ImagePart {
    pub image: Option<Vec<u8>>,
    pub image_url: Option<String>,
    pub mime_type: Option<String>,
}

pub struct FilePart {
    pub file_name: Option<String>,
    pub file_content: Option<Vec<u8>>,
    pub file_url: Option<String>,
    pub mime_type: Option<String>,
}

pub enum UserContentParts {
    Text(TextPart),
    Image(ImagePart),
    File(FilePart),
}

pub enum UserContent {
    Text(String),
    Parts(Vec<UserContentParts>),
}

pub struct ReasoningPart {
    pub text: String,
    pub signature: Option<String>,
}

pub struct RedactedReasoningPart {
    pub data: String,
}

pub struct ToolCallPart {
    pub tool_call_id: String,
    pub tool_name: String,
    pub args: String,
}

pub enum AssistantContentParts {
    Text(TextPart),
    File(FilePart),
    Reasoning(ReasoningPart),
    RedactedReasoning(RedactedReasoningPart),
    ToolCall(ToolCallPart),
}

pub enum AssistantContent {
    Text(String),
    Parts(Vec<AssistantContentParts>),
}

pub struct ToolResultPart {
    pub tool_call_id: String,
    pub tool_name: String,
    pub result: String,
    pub is_error: Option<bool>,
}
