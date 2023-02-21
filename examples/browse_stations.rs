use tunein::TuneInClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TuneInClient::new();
    let results = client.browse_by_id("s269487").await?;
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
