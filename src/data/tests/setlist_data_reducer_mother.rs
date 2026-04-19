use crate::data::models::args::StreamingService;
use crate::data::models::meta_data::{ArtistAnalysis, ArtistAnalysisCollection};
use crate::data::models::song_stats::SongStats;
use crate::data::setlist_data_reducer::SetlistDataReducer;
use std::collections::HashMap;

pub struct SetlistDataReducerMotherObject;

impl SetlistDataReducerMotherObject {
    pub fn reducer(song_stats_by_name: HashMap<String, SongStats>) -> SetlistDataReducer {
        let average_songs_per_setlist = song_stats_by_name.len() as f32;
        Self::reducer_with_average(song_stats_by_name, average_songs_per_setlist)
    }

    pub fn reducer_with_average(
        song_stats_by_name: HashMap<String, SongStats>,
        average_songs_per_setlist: f32,
    ) -> SetlistDataReducer {
        SetlistDataReducer::new(
            "My Playlist".to_string(),
            StreamingService::Spotify,
            ArtistAnalysisCollection {
                artist_analyses: vec![ArtistAnalysis {
                    artist_name: "Test Artist".to_string(),
                    song_stats_by_name,
                    average_songs_per_setlist,
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
