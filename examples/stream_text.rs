//! Create a completion stream for the provided prompt and parameters.

use fieri::{
    completion::{create_with_stream, Completion, CompletionParamBuilder},
    Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let param = CompletionParamBuilder::new("ada")
        .prompt("unnecessarily lo")
        .temperature(0.5)
        .build()?;

    let mut resp = create_with_stream(&client, &param).await?;

    while let Some(chunk) = resp.chunk().await? {
        if chunk.to_vec() == b"data: [DONE]\n\n" {
            break;
        }

        let v: Completion = serde_json::from_slice(&chunk[5..])?;
        v.choices.iter().for_each(|c| println!("{:?}", c.text));
    }

    Ok(())
}
