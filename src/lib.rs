use std::time::Duration;

use surf::{Client, Config, Url};
use types::{Category, CategoryDetails, CategoryResponse, CategoryTrait, SearchResponse, Station};

use crate::types::{CategoriesResponse, SearchResult};

pub mod types;

pub const BASE_URL: &str = "https://opml.radiotime.com";
pub struct TuneInClient {
    client: Client,
}

impl TuneInClient {
    pub fn new() -> Self {
        let client = Config::new()
            .set_timeout(Some(Duration::from_secs(5)))
            .set_base_url(Url::parse(BASE_URL).unwrap())
            .try_into()
            .unwrap();
        Self { client }
    }

    pub async fn browse(
        &self,
        category: Option<Category>,
    ) -> Result<Vec<CategoryDetails>, surf::Error> {
        let path = match category {
            Some(c) => format!("Browse.ashx?c={}&render=json", c.to_id()),
            None => "Browse.ashx?render=json".to_string(),
        };
        let response = self
            .client
            .get(path)
            .recv_json::<CategoriesResponse>()
            .await?;
        Ok(response.body)
    }

    pub async fn browse_by_id(&self, id: &str) -> Result<Vec<Station>, surf::Error> {
        let response = self
            .client
            .get(format!("Browse.ashx?id={}&render=json", id))
            .recv_json::<CategoryResponse>()
            .await?;
        Ok(response.body)
    }

    pub async fn search(&self, station: &str) -> Result<Vec<SearchResult>, surf::Error> {
        let response = self
            .client
            .get(format!("Search.ashx?query={}&render=json", station))
            .recv_json::<SearchResponse>()
            .await?;
        Ok(response.body)
    }

    pub async fn get_categories(&self) -> Result<Vec<CategoryDetails>, surf::Error> {
        let response = self
            .client
            .get("?render=json")
            .recv_json::<CategoriesResponse>()
            .await?;
        Ok(response.body)
    }
}
