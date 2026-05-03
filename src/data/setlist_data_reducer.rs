use crate::data::models::args::StreamingService;
use crate::data::models::meta_data::ArtistAnalysisCollection;
use crate::data::models::playlist_data::{ArtistPlaylist, PlaylistData};
use crate::data::models::song_stats::SongStats;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum RoleBucket {
    Opener,
    Regular,
    Closer,
    Encore,
}

pub struct SetlistDataReducer {
    playlist_name: String,
    streaming_service: StreamingService,
    analysis_collection: ArtistAnalysisCollection,
    simple_mode_flag: bool,
}

impl SetlistDataReducer {
    const MIN_ROLE_PLAYS: u32 = 3;
    const EXCEPTIONAL_ROLE_RATE: f32 = 0.60;
    const TARGET_SETLIST_SIZE_OFFSET: usize = 3; // This Algorithm tends to underestimate the number of songs due to smaller Festival Setlists and smaller Shows so we add this here to increase the number of songs selected for the playlist

    pub fn new(
        playlist_name: String,
        streaming_service: StreamingService,
        analysis_collection: ArtistAnalysisCollection,
        simple_mode_flag: bool,
    ) -> Self {
        Self {
            playlist_name,
            streaming_service,
            analysis_collection,
            simple_mode_flag,
        }
    }

    pub fn reduce(&self) -> PlaylistData {
        let artist_playlists = match self.simple_mode_flag {
            true => self.invoke_simple_mode_reduction(),
            false => self.invoke_normal_mode_reduction(),
        };

        PlaylistData {
            playlist_name: self.playlist_name.clone(),
            platforms: vec![self.streaming_service.clone()],
            artist_playlists,
        }
    }

    fn invoke_simple_mode_reduction(&self) -> Vec<ArtistPlaylist> {
        self.analysis_collection
            .artist_analyses
            .par_iter()
            .map(|artist_analysis| {
                let artist = artist_analysis.artist_name.clone();
                let songs_by_position = Self::map_song_stats_to_playlist_slots();

                ArtistPlaylist {
                    artist,
                    songs_by_position,
                }
            })
            .collect()
    }

    fn invoke_normal_mode_reduction(&self) -> Vec<ArtistPlaylist> {
        self.analysis_collection
            .artist_analyses
            .par_iter()
            .map(|artist_analysis| {
                let artist = artist_analysis.artist_name.clone();
                let songs_by_position = Self::map_song_stats_to_playlist_slots(
                    artist_analysis.song_stats_by_name.clone(),
                    artist_analysis.average_songs_per_setlist,
                );

                ArtistPlaylist {
                    artist,
                    songs_by_position,
                }
            })
            .collect()
    }

    fn map_song_stats_to_playlist_slots(
        song_stats_by_name: HashMap<String, SongStats>,
        average_songs_per_setlist: f32,
    ) -> BTreeMap<usize, String> {
        let target_setlist_size = Self::target_setlist_size(average_songs_per_setlist);
        let selected_song_stats =
            Self::select_most_played_songs(song_stats_by_name, target_setlist_size);
        let ranked_songs = Self::rank_songs_for_setlist(selected_song_stats);

        ranked_songs
            .into_iter()
            .enumerate()
            .map(|(index, (song_name, _))| (index + 1, song_name))
            .collect()
    }

    fn target_setlist_size(average_songs_per_setlist: f32) -> usize {
        let rounded_up_setlist_size = Self::rounded_up_setlist_size(average_songs_per_setlist);

        if rounded_up_setlist_size == 0 {
            0
        } else {
            rounded_up_setlist_size + Self::TARGET_SETLIST_SIZE_OFFSET
        }
    }

    fn select_most_played_songs(
        song_stats_by_name: HashMap<String, SongStats>,
        target_setlist_size: usize,
    ) -> HashMap<String, SongStats> {
        if target_setlist_size == 0 {
            return HashMap::new();
        }

        let mut songs_sorted_by_plays: Vec<(String, SongStats)> =
            song_stats_by_name.into_iter().collect();

        songs_sorted_by_plays.sort_by(|(_, a), (_, b)| {
            b.total_plays
                .cmp(&a.total_plays)
                .then_with(|| Self::compare_for_setlist_order(a, b))
                .then_with(|| a.song_name.cmp(&b.song_name))
        });

        songs_sorted_by_plays
            .into_iter()
            .take(target_setlist_size)
            .collect()
    }

    fn rounded_up_setlist_size(average_songs_per_setlist: f32) -> usize {
        if average_songs_per_setlist.is_finite() && average_songs_per_setlist > 0.0 {
            average_songs_per_setlist.ceil() as usize
        } else {
            0
        }
    }

    fn rank_songs_for_setlist(
        song_stats_by_name: HashMap<String, SongStats>,
    ) -> Vec<(String, SongStats)> {
        let mut ranked_songs: Vec<(String, SongStats)> = song_stats_by_name.into_iter().collect();

        ranked_songs.sort_by(|(_, a), (_, b)| {
            Self::compare_for_setlist_order(a, b)
                .then_with(|| b.total_plays.cmp(&a.total_plays))
                .then_with(|| a.song_name.cmp(&b.song_name))
        });

        ranked_songs
    }

    fn compare_for_setlist_order(a: &SongStats, b: &SongStats) -> Ordering {
        let a_bucket = Self::role_bucket_for_song(a);
        let b_bucket = Self::role_bucket_for_song(b);

        a_bucket
            .cmp(&b_bucket)
            .then_with(|| Self::compare_within_bucket(a, b, a_bucket))
    }

    fn compare_within_bucket(a: &SongStats, b: &SongStats, bucket: RoleBucket) -> Ordering {
        match bucket {
            RoleBucket::Opener => Self::opener_rate(b)
                .partial_cmp(&Self::opener_rate(a))
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    Self::mean_position(a)
                        .partial_cmp(&Self::mean_position(b))
                        .unwrap_or(Ordering::Equal)
                }),
            RoleBucket::Regular => Self::mean_position(a)
                .partial_cmp(&Self::mean_position(b))
                .unwrap_or(Ordering::Equal),
            RoleBucket::Closer => Self::closer_rate(b)
                .partial_cmp(&Self::closer_rate(a))
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    Self::mean_position(b)
                        .partial_cmp(&Self::mean_position(a))
                        .unwrap_or(Ordering::Equal)
                }),
            RoleBucket::Encore => Self::encore_rate(b)
                .partial_cmp(&Self::encore_rate(a))
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    Self::mean_position(b)
                        .partial_cmp(&Self::mean_position(a))
                        .unwrap_or(Ordering::Equal)
                }),
        }
    }

    fn role_bucket_for_song(stats: &SongStats) -> RoleBucket {
        if stats.total_plays < Self::MIN_ROLE_PLAYS {
            return RoleBucket::Regular;
        }

        let opener_rate = Self::opener_rate(stats);
        let closer_rate = Self::closer_rate(stats);
        let encore_rate = Self::encore_rate(stats);

        let (bucket, rate) = if opener_rate >= closer_rate && opener_rate >= encore_rate {
            (RoleBucket::Opener, opener_rate)
        } else if closer_rate >= encore_rate {
            (RoleBucket::Closer, closer_rate)
        } else {
            (RoleBucket::Encore, encore_rate)
        };

        if rate >= Self::EXCEPTIONAL_ROLE_RATE {
            bucket
        } else {
            RoleBucket::Regular
        }
    }

    fn mean_position(stats: &SongStats) -> f32 {
        stats
            .mean_positions_played
            .last()
            .copied()
            .unwrap_or(f32::MAX)
    }

    fn opener_rate(stats: &SongStats) -> f32 {
        if stats.total_plays == 0 {
            return 0.0;
        }

        stats.opener_count as f32 / stats.total_plays as f32
    }

    fn closer_rate(stats: &SongStats) -> f32 {
        if stats.total_plays == 0 {
            return 0.0;
        }

        stats.closer_count as f32 / stats.total_plays as f32
    }

    fn encore_rate(stats: &SongStats) -> f32 {
        if stats.total_plays == 0 {
            return 0.0;
        }

        stats.encore_count as f32 / stats.total_plays as f32
    }
}
