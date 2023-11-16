use fieri::{
    chat::chat,
    types::{ChatMessageBuilder, ChatParamBuilder, ChatRole},
    Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let message = ChatMessageBuilder::new(ChatRole::User, "Hello!").build()?;
    let param = ChatParamBuilder::new("gpt-3.5-turbo", vec![message]).build()?;

    let resp = chat(&client, &param).await?;
    println!("Generated text: {:#?}", resp);

    Ok(())
}
