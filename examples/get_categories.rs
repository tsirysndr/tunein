use tunein::TuneInClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TuneInClient::new();
    let categories = client.get_categories().await?;
    println!("{}", serde_json::to_string_pretty(&categories)?);
    Ok(())
}
