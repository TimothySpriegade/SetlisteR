use crate::Args;
use crate::StreamingService;

pub struct ArgsMotherObject {
    artists: String,
    playlist_name: Option<String>,
    service: StreamingService,
    page_depth: u16,
}

impl ArgsMotherObject {
    pub fn default() -> Self {
        Self {
            artists: "Artist 1".to_string(),
            playlist_name: None,
            service: StreamingService::Spotify,
            page_depth: 1,
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

    pub fn build(self) -> Args {
        Args {
            artists: self.artists,
            playlist_name: self.playlist_name,
            service: self.service,
            page_depth: self.page_depth,
        }
    }
}

