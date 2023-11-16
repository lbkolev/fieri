use crate::{
    types::{Chat, ChatParam},
    Client, Result,
};

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
mod tests {}
