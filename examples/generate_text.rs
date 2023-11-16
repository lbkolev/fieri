use fieri::{completion::create, types::CompletionParamBuilder, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let param = CompletionParamBuilder::new("ada")
        .prompt(vec![
            "Generate a plot for an absurd interstellar parody.".into()
        ])
        .max_tokens(500)
        .temperature(0.9)
        .top_p(1.0)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .build()?;

    let resp = create(&client, &param).await?;
    println!("Generated text: {:#?}", resp);

    Ok(())
}
