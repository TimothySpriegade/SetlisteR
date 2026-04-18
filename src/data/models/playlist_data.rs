use crate::data::models::args::StreamingService;
use std::collections::BTreeMap;

pub struct PlaylistData {
    pub playlist_name: String,
    pub platforms: Vec<StreamingService>,
    pub artist_playlists: Vec<ArtistPlaylist>,
}

pub struct ArtistPlaylist {
    pub artist: String,
    pub songs_by_position: BTreeMap<usize, String>,
}
