use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetlistResponse {
    pub setlist: Vec<Setlist>,
    pub total: u32,
    pub page: u32,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setlist {
    pub artist: Artist,
    pub venue: Venue,
    pub tour: Option<Tour>,
    pub set: Vec<Set>,
    pub info: Option<String>,
    pub url: String,
    pub id: String,
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "eventDate")]
    pub event_date: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub mbid: String,
    pub name: String,
    #[serde(rename = "sortName")]
    pub sort_name: String,
    pub disambiguation: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Venue {
    pub id: String,
    pub name: String,
    pub url: String,
    pub city: Option<City>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub id: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "stateCode")]
    pub state_code: Option<String>,
    pub country: Option<Country>,
    pub coords: Option<Coords>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coords {
    pub lat: Option<f64>,
    pub long: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tour {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Set {
    pub name: Option<String>,
    pub encore: Option<u32>,
    pub song: Vec<Song>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub name: Option<String>,
    pub info: Option<String>,
    pub cover: Option<Artist>,
    pub tape: Option<bool>,
}
