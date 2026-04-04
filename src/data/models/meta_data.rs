use std::collections::HashMap;
use crate::data::models::song_stats::SongStats;


pub struct CollectedData {
    pub collected_meta_data: Vec<MetaData>,
}
#[derive(Debug, Clone, Default)]
pub struct MetaData {
    pub artist_name: String,
    pub song_stats: HashMap<String, SongStats>,
    pub mean_song_count: f32,
}