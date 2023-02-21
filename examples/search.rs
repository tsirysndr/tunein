use tunein::TuneInClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TuneInClient::new();
    let results = client.search("alternativeradio.us").await?;
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
