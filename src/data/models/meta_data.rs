use crate::data::models::song_stats::SongStats;
use std::collections::HashMap;

pub struct ArtistAnalysisCollection {
    pub artist_analyses: Vec<ArtistAnalysis>,
}

#[derive(Debug, Clone, Default)]
pub struct ArtistAnalysis {
    pub artist_name: String,
    pub song_stats_by_name: HashMap<String, SongStats>,
    pub average_songs_per_setlist: f32,
}
