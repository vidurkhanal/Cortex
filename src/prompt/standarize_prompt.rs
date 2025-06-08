use crate::errors::ModelError;

use super::{
    content_part::{AssistantContent, UserContent},
    CoreMessage, CoreUserMessage, Prompt,
};

pub enum StandardizedPromptKind {
    Prompt,
    Messages,
}

pub struct StandardizedPrompt {
    kind: StandardizedPromptKind,
    system: Option<String>,
    messages: Vec<CoreMessage>,
}

impl TryFrom<Prompt> for StandardizedPrompt {
    type Error = ModelError;
    fn try_from(prompt: Prompt) -> Result<Self, Self::Error> {
        if prompt.prompt.is_none() && prompt.messages.is_none() {
            return Err(ModelError::InvalidPrompt(
                "Prompt must contain either a prompt or messages".to_string(),
            ));
        }

        if prompt.prompt.is_some() && prompt.messages.is_some() {
            return Err(ModelError::InvalidPrompt(
                "Prompt cannot contain both a prompt and messages".to_string(),
            ));
        }

        if prompt.prompt.is_some() {
            return Ok(StandardizedPrompt {
                kind: StandardizedPromptKind::Prompt,
                system: Some(prompt.system),
                messages: vec![CoreMessage::User(CoreUserMessage {
                    content: UserContent::Text(prompt.prompt.unwrap()),
                })],
            });
        }

        if prompt.messages.is_some() {
            let messages = prompt.messages.unwrap();

            //TODO: not sure what UI messages are, but they are not supported yet
            // let promptType = detect_prompt_type(messages)

            if messages.is_empty() {
                return Err(ModelError::InvalidPrompt(
                    "Messages cannot be empty".to_string(),
                ));
            }

            return Ok(StandardizedPrompt {
                kind: StandardizedPromptKind::Messages,
                system: Some(prompt.system),
                messages,
            });
        }

        Err(ModelError::Other(
            "Prompt must contain either a prompt or messages".to_string(),
        ))
    }
}

enum PromptType {
    UIMessages,
    Messages,
    Other,
}

fn detect_prompt_type(messages: &[CoreMessage]) -> PromptType {
    if messages.is_empty() {
        return PromptType::Messages;
    }

    let characterstics: Vec<PromptCharacteristics> = messages
        .iter()
        .map(detect_single_message_characteristics)
        .collect();

    if characterstics
        .iter()
        .any(|c| *c == PromptCharacteristics::HasUISpecificParts)
    {
        return PromptType::UIMessages;
    }

    PromptType::Messages
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PromptCharacteristics {
    HasUISpecificParts,
    HasCoreSpecificParts,
    Message,
}

fn detect_single_message_characteristics(message: &CoreMessage) -> PromptCharacteristics {
    match message {
        CoreMessage::System(_) => PromptCharacteristics::Message,
        CoreMessage::User(user_message) => match user_message.content {
            UserContent::Text(_) => PromptCharacteristics::Message,
            UserContent::Parts(_) => PromptCharacteristics::HasCoreSpecificParts,
        },
        CoreMessage::Assistant(assistant_message) => match assistant_message.content {
            AssistantContent::Text(_) => PromptCharacteristics::Message,
            AssistantContent::Parts(_) => PromptCharacteristics::HasCoreSpecificParts,
        },
        CoreMessage::Tool(tool_message) => {
            if tool_message.content.is_empty() {
                PromptCharacteristics::Message
            } else {
                PromptCharacteristics::HasCoreSpecificParts
            }
        }
    }
}

