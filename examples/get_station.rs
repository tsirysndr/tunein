use tunein::TuneInClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TuneInClient::new();
    let station_links = client.get_station("s221580").await?;
    println!("{}", serde_json::to_string_pretty(&station_links)?);
    Ok(())
}
