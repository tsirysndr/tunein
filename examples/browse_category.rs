use tunein::{types::Category, TuneInClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TuneInClient::new();
    let results = client.browse(Some(Category::Music)).await?;
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
