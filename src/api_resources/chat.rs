use clap::Parser;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;



use crate::{api_resources::TokenUsage, utils::is_false, Client, Result};

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize, Parser)]
#[builder(default, setter(into, strip_option))]
pub struct ChatParam {
    /// A list of messages describing the conversation so far.
    #[clap(short, long, required = true, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub messages: Vec<ChatMessage>,

    /// ID of the model to use.
    #[clap(long, default_value = "gpt-3.5-turbo")]
    pub model: String,

    /// Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub frequency_penalty: Option<f32>,

    /// The maximum number of tokens to generate in the chat completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub max_tokens: Option<u32>,

    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub n: Option<u32>,

    /// Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub presence_penalty: Option<f32>,

    /// This feature is in Beta.
    ///
    /// If specified, our system will make a best effort to sample deterministically,
    /// such that repeated requests with the same seed and parameters should return the same result.
    /// Determinism is not guaranteed, and you should refer to the system_fingerprint response parameter to monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub seed: Option<u64>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub stop: Option<String>,

    /// If set, partial message deltas will be sent, like in ChatGPT.
    #[serde(skip_serializing_if = "is_false")]
    #[clap(long)]
    pub stream: bool,

    /// What sampling temperature to use, between 0 and 2.
    /// Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub top_p: Option<f32>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct ChatMessage {
    /// The role of the author of this message. One of system, user, or assistant.
    pub role: ChatRole,

    /// The contents of the message.
    pub content: String,

    /// The name of the author of this message. May contain a-z, A-Z, 0-9, and underscores, with a maximum length of 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Function,
}

impl Default for ChatRole {
    fn default() -> Self {
        Self::User
    }
}

impl From<String> for ChatRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "system" => Self::System,
            "user" => Self::User,
            "assistant" => Self::Assistant,
            "function" => Self::Function,
            _ => Self::User,
        }
    }
}

impl ChatMessageBuilder {
    pub fn new(role: impl Into<ChatRole>, content: impl Into<String>) -> Self {
        Self {
            role: Some(role.into()),
            content: Some(content.into()),
            ..Self::default()
        }
    }
}

/// takes strings in the form of <role>:<content>:<name> and parses it into ChatMessage

impl From<String> for ChatMessage {
    fn from(s: String) -> Self {
        let mut split = s.split(':');
        let role: ChatRole = split.next().unwrap_or("user").to_string().into();
        let content = split.next().unwrap_or("");
        let name = split.next().unwrap_or("");

        Self {
            role: role.into(),
            content: content.to_string(),
            name: Some(name.to_string()),
        }
    }
}

impl ChatParamBuilder {
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: Some(model.into()),
            messages: Some(messages),
            ..Self::default()
        }
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Chat {
    id: String,
    object: String,
    created: i64,
    choices: Vec<ChatChoice>,

    usage: TokenUsage,
}

pub async fn chat(client: &Client, param: &ChatParam) -> Result<Chat> {
    client.chat(param).await
}

impl Client {
    async fn chat(&self, param: &ChatParam) -> Result<Chat> {
        self.post::<ChatParam, Chat>("chat/completions", Some(param))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_deserialization() {
        let param: ChatParam = serde_json::from_str(
            r#"
            {
                "model": "gpt-3.5-turbo",
                "messages": [{"role": "user", "content": "Hello!"}]
            }
            "#,
        )
        .unwrap();

        let resp: Chat = serde_json::from_str(
            r#"
            {
                "id": "chatcmpl-123",
                "object": "chat.completion",
                "created": 1677652288,
                "choices": [{
                  "index": 0,
                  "message": {
                    "role": "assistant",
                    "content": "\n\nHello there, how may I assist you today?"
                  },
                  "finish_reason": "stop"
                }],
                "usage": {
                  "prompt_tokens": 9,
                  "completion_tokens": 12,
                  "total_tokens": 21
                }
              }
            "#,
        )
        .unwrap();

        assert_eq!(param.model, "gpt-3.5-turbo");
        assert_eq!(param.messages.len(), 1);
        assert_eq!(resp.choices.len(), 1);
        assert_eq!(
            resp.choices[0].message.content,
            "\n\nHello there, how may I assist you today?"
        );
        assert_eq!(resp.choices[0].finish_reason, Some("stop".to_string()));
        assert_eq!(resp.usage.prompt_tokens, 9);
    }
}
