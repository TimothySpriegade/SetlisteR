use crate::data::models::args::StreamingService;
use std::collections::BTreeMap;

pub struct PlaylistData {
    pub playlist_name: String,
    pub platforms: Vec<StreamingService>,
    pub artist_song_data: Vec<ArtistSongData>,
}

pub struct ArtistSongData {
    pub artist: String,
    pub songs: BTreeMap<usize, String>, // song position -> song name
}
