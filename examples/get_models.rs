use fieri::{model::retrieve, Client, Error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let resp = retrieve(&client, "davinci").await?;
    println!("Resp {:#?}", resp);
    Ok(())
}
