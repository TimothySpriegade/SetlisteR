use crate::data::models::args::StreamingService;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct PlaylistData {
    pub playlist_name: String,
    pub platforms: Vec<StreamingService>,
    pub artist_playlists: Vec<ArtistPlaylist>,
}

#[derive(Serialize)]
pub struct ArtistPlaylist {
    pub artist: String,
    pub songs_by_position: BTreeMap<usize, String>,
}
