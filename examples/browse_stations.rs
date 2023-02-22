use tunein::TuneInClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TuneInClient::new();
    let category_id = "c57943";
    let results = client.browse_by_id(category_id).await?;
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
