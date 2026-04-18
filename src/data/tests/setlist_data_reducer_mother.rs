use crate::data::models::args::StreamingService;
use crate::data::models::meta_data::{CollectedData, MetaData};
use crate::data::models::song_stats::SongStats;
use crate::data::setlist_data_reducer::SetlistDataReducer;
use std::collections::HashMap;

pub struct SetlistDataReducerMotherObject;

impl SetlistDataReducerMotherObject {
    pub fn reducer(song_stats: HashMap<String, SongStats>) -> SetlistDataReducer {
        SetlistDataReducer::new(
            "My Playlist".to_string(),
            StreamingService::Spotify,
            CollectedData {
                collected_meta_data: vec![MetaData {
                    artist_name: "Test Artist".to_string(),
                    song_stats,
                    mean_song_count: 0.0,
                }],
            },
        )
    }

    pub fn song_stats(
        song_name: &str,
        total_plays: u32,
        opener_count: u32,
        closer_count: u32,
        encore_count: u32,
        mean_position: f32,
    ) -> SongStats {
        SongStats {
            song_name: song_name.to_string(),
            total_plays,
            opener_count,
            closer_count,
            encore_count,
            positions_played: Vec::new(),
            mean_positions_played: vec![mean_position],
            last_played: String::new(),
        }
    }
}
