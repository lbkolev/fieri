use fieri::{model::retrieve, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let resp = retrieve(&client, "davinci").await?;
    println!("Resp {:#?}", resp);
    Ok(())
}
