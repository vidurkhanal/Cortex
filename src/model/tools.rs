use std::collections::HashMap;

use crate::prompt::CoreMessage;

pub struct ToolExecutionOptions {
    tool_call_id: String,
    messages: Vec<CoreMessage>,
}

pub struct Tool {
    pub description: Option<String>,
    pub execution_options: Option<ToolExecutionOptions>,
}

pub type ToolSet = HashMap<String, Tool>;
