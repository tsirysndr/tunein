# TuneIn Rust Client Library

<p>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" />
  </a>
  <a href="https://crates.io/crates/tunein" target="_blank">
    <img src="https://img.shields.io/crates/v/tunein.svg" />
  </a>
  
  <a href="https://crates.io/crates/tunein" target="_blank">
    <img src="https://img.shields.io/crates/dr/tunein" />
  </a>
  
  <a href="https://docs.rs/tunein" target="_blank">
    <img src="https://docs.rs/tunein/badge.svg" />
  </a>
</p>


<img src="./logo.png" />

This is a Rust client library for the [TuneIn Radio](https://tunein.com/) API.

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
tunein = "0.1"
```

## Usage

Search for a station:

```rust
use tunein::TuneInClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TuneInClient::new();
    let results = client.search("alternativeradio.us").await?;
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
```

See the [examples](./examples) directory for more examples.

## Features

- [x] Search
- [x] Browse categories
- [x] Browse stations by category

## License
MIT

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
