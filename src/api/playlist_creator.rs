use crate::api::ytmusic_sidecar::YTMusicSidecar;
use crate::data::models::args::StreamingService;
use crate::data::models::playlist_data::PlaylistData;
use crate::secrets_manager::secrets_manager::SecretsManager;

pub struct PlaylistCreator<'a> {
    secret_manager: &'a SecretsManager,
}

impl<'a> PlaylistCreator<'a> {
    pub fn new(secret_manager: &'a SecretsManager) -> Self {
        Self { secret_manager }
    }

    pub fn create(&self, playlist_data: &PlaylistData, service: &StreamingService) {
        match service {
            StreamingService::Spotify => {
                println!("Spotify integration is not yet implemented.");
            }
            StreamingService::YoutubeMusic => {
                self.create_youtube_music_playlist(playlist_data);
            }
        }
    }

    fn resolve_youtube_oauth_json(&self) -> String {
        if let Some(oauth_json) = self.secret_manager.get_youtube_oauth_json() {
            return oauth_json;
        }

        let headers = self.secret_manager.get_youtube_headers().expect(
            "YouTube browser credentials/headers not found. Provide --youtube-headers to log in.",
        );

        let sidecar = YTMusicSidecar::initialize().expect("Failed to initialize Python sidecar");

        let oauth_json = sidecar
            .login(&headers)
            .expect("YouTube Music login flow failed");

        self.secret_manager
            .set_youtube_oauth_json(oauth_json.clone());

        oauth_json
    }

    fn create_youtube_music_playlist(&self, playlist_data: &PlaylistData) {
        println!("\nInitializing YouTube Music sidecar...");
        let sidecar = YTMusicSidecar::initialize().expect("Failed to initialize Python sidecar");

        let oauth_json = self.resolve_youtube_oauth_json();

        sidecar
            .create_playlist(playlist_data, &oauth_json)
            .expect("Failed to create playlist");
    }
}
