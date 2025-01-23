use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub enum Category {
    ByLocation,
    ByLanguage,
    Sports,
    Talk,
    Music,
    LocalRadio,
    Podcasts,
}

pub trait CategoryTrait {
    fn to_id(&self) -> String;
}

impl CategoryTrait for Category {
    fn to_id(&self) -> String {
        match self {
            Category::ByLocation => "r0".to_string(),
            Category::ByLanguage => "lang".to_string(),
            Category::Sports => "sports".to_string(),
            Category::Talk => "talk".to_string(),
            Category::Music => "music".to_string(),
            Category::LocalRadio => "local".to_string(),
            Category::Podcasts => "podcast".to_string(),
        }
    }
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Category::ByLocation => "By Location".to_string(),
            Category::ByLanguage => "By Language".to_string(),
            Category::Sports => "Sports".to_string(),
            Category::Talk => "Talk".to_string(),
            Category::Music => "Music".to_string(),
            Category::LocalRadio => "Local Radio".to_string(),
            Category::Podcasts => "Podcasts".to_string(),
        }
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "By Location" => Ok(Category::ByLocation),
            "By Language" => Ok(Category::ByLanguage),
            "Sports" => Ok(Category::Sports),
            "Talk" => Ok(Category::Talk),
            "Music" => Ok(Category::Music),
            "Local Radio" => Ok(Category::LocalRadio),
            "Podcasts" => Ok(Category::Podcasts),
            _ => Err(format!("{} is not a valid category", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub text: String,
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Box<SearchResult>>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Head {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CategoriesResponse {
    pub head: Head,
    pub body: Vec<CategoryDetails>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CategoryResponse {
    pub head: Head,
    pub body: Vec<Station>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub element: String,
    pub text: String,
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reliability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playing_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub now_playing_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Box<SearchResult>>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SearchResponse {
    pub head: Head,
    pub body: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Station {
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<String>,
    pub element: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub now_playing_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playing_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reliability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtext: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Box<Station>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StationLinkDetails {
    pub bitrate: u32,
    pub element: String,
    pub is_ad_clipped_content_enabled: String,
    pub is_direct: bool,
    pub is_hls_advanced: String,
    pub live_seek_stream: String,
    pub media_type: String,
    pub player_height: u32,
    pub player_width: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    pub position: u32,
    pub reliability: u32,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StationResponse {
    pub head: Head,
    pub body: Vec<StationLinkDetails>,
}
