use crate::{
    types::{Chat, ChatParam},
    Client, Result,
};

pub async fn chat(client: &Client, param: impl Into<&ChatParam>) -> Result<Chat> {
    client.chat(param.into()).await
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
    use crate::types::{ChatMessageBuilder, ChatParamBuilder, ChatRole};

    use mockito;
    use serde_json::json;
    use url::Url;

    #[tokio::test]
    async fn test_invalid_role_woops() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut server = mockito::Server::new();
        let client = Client::mock_new(Url::parse(
            format!("http:{}", server.host_with_port()).as_str(),
        )?);

        server.mock("POST", "/chat/completions")
            .match_body(r#"{"model":"gpt-4","messages":[{"role":"woops","content":"You are a helpful assistant."},{"role":"user","content":"Hello!"}]}"#)
            .with_status(400)
            .with_body(r#"{"error":{"message":"'woops' is not one of ['system', 'assistant', 'user', 'function'] - 'messages.0.role'","type":"invalid_request_error","param":null,"code":null}}"#)
            .create();

        let param = ChatParamBuilder::new(
            "gpt-4",
            vec![ChatMessageBuilder::new(
                ChatRole::User,
                "When is humanity going to reach Kardashev Type I?",
            )
            .build()?],
        )
        //.add_message("woops", "You are a helpful assistant.")
        //.add_message("user", "Hello!")
        .build()?;

        let response = chat(&client, &param).await?;
        assert_eq!(response.is_err(), true);

        Ok(())
    }

    #[tokio::test]
    async fn test_valid_request_with_system() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let mut server = mockito::Server::new();
        let client = Client::mock_new(Url::parse(
            format!("http:{}", server.host_with_port()).as_str(),
        )?);

        server.mock("POST", "/chat/completions")
            .match_body(r#"{"model":"gpt-4","messages":[{"role":"system","content":"You are a helpful assistant."},{"role":"user","content":"Hello!"}]}"#)
            .with_status(200)
            .with_body(r#"{"id":"chatcmpl-8LYzlrh0kFsXQQkryg4BGkJlIxuen","object":"chat.completion","created":1700150101,"model":"gpt-4-0613","choices":[{"index":0,"message":{"role":"assistant","content":"Hello! How can I assist you today?"},"finish_reason":"stop"}],"usage":{"prompt_tokens":19,"completion_tokens":9,"total_tokens":28}}"#)
            .create();

        let param = ChatParamBuilder::new(
            "gpt-4",
            vec![ChatMessageBuilder::new(
                ChatRole::User,
                "When is humanity going to reach Kardashev Type I?",
            )
            .build()?],
        )
        .build()?;

        let response = chat(&client, &param).await?;
        assert!(response.is_ok());

        Ok(())
    }

    /*
    #[tokio::test]
    async fn test_invalid_function_role_without_name() {
        let _m = mock("POST", "/chat/completions")
            .match_body(r#"{"model":"gpt-4","messages":[{"role":"function","content":"You are a helpful assistant."},{"role":"user","content":"Hello!"}]}"#)
            .with_status(400)
            .with_body(r#"{"error":{"message":"Missing parameter 'name': messages with role 'function' must have a 'name'.","type":"invalid_request_error","param":"messages.[0].name","code":null}}"#)
            .create();

        let response = chat(&server_url(), "function", "You are a helpful assistant.").await;
        assert_eq!(response.is_err(), true);
    }
    */
}
