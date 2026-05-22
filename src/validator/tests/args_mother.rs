use crate::Args;
use crate::data::models::args::StreamingService;

pub struct ArgsMotherObject {
    artists: String,
    playlist_name: Option<String>,
    service: StreamingService,
    page_depth: u16,
    setlist_api_key: Option<String>,
    youtube_client_id: Option<String>,
    youtube_client_secret: Option<String>,
}

impl ArgsMotherObject {
    pub fn default() -> Self {
        Self {
            artists: "Artist 1".to_string(),
            playlist_name: None,
            service: StreamingService::Spotify,
            page_depth: 1,
            setlist_api_key: None,
            youtube_client_id: None,
            youtube_client_secret: None,
        }
    }

    pub fn with_artists(mut self, artists: &str) -> Self {
        self.artists = artists.to_string();
        self
    }

    pub fn with_playlist_name(mut self, playlist_name: Option<&str>) -> Self {
        self.playlist_name = playlist_name.map(|s| s.to_string());
        self
    }

    pub fn with_service(mut self, service: StreamingService) -> Self {
        self.service = service;
        self
    }

    pub fn with_page_depth(mut self, page_depth: u16) -> Self {
        self.page_depth = page_depth;
        self
    }

    pub fn with_setlist_api_key(mut self, setlist_api_key: &str) -> Self {
        self.setlist_api_key = Some(setlist_api_key.to_string());
        self
    }

    pub fn with_youtube_client_id(mut self, youtube_client_id: &str) -> Self {
        self.youtube_client_id = Some(youtube_client_id.to_string());
        self
    }

    pub fn with_youtube_client_secret(mut self, youtube_client_secret: &str) -> Self {
        self.youtube_client_secret = Some(youtube_client_secret.to_string());
        self
    }

    pub fn build(self) -> Args {
        Args {
            artists: self.artists,
            playlist_name: self.playlist_name,
            service: self.service,
            page_depth: self.page_depth,
            setlist_api_key: self.setlist_api_key,
            youtube_client_id: self.youtube_client_id,
            youtube_client_secret: self.youtube_client_secret,
        }
    }
}
