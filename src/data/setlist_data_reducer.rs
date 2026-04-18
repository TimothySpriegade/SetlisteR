use crate::data::models::args::StreamingService;
use crate::data::models::meta_data::ArtistAnalysisCollection;
use crate::data::models::playlist_data::{ArtistPlaylist, PlaylistData};
use crate::data::models::song_stats::SongStats;
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
}

impl SetlistDataReducer {
    const MIN_ROLE_PLAYS: u32 = 3;
    const EXCEPTIONAL_ROLE_RATE: f32 = 0.60;

    pub fn new(
        playlist_name: String,
        streaming_service: StreamingService,
        analysis_collection: ArtistAnalysisCollection,
    ) -> Self {
        Self {
            playlist_name,
            streaming_service,
            analysis_collection,
        }
    }

    pub fn reduce(&self) -> PlaylistData {
        let mut playlist_data = PlaylistData {
            playlist_name: self.playlist_name.clone(),
            platforms: vec![self.streaming_service.clone()],
            artist_playlists: Vec::new(),
        };

        let mut artist_playlists = Vec::new();

        for artist_analysis in &self.analysis_collection.artist_analyses {
            let artist = artist_analysis.artist_name.clone();
            let songs_by_position =
                Self::map_song_stats_to_playlist_slots(artist_analysis.song_stats_by_name.clone());

            artist_playlists.push(ArtistPlaylist {
                artist,
                songs_by_position,
            });
        }
        playlist_data.artist_playlists = artist_playlists;
        playlist_data
    }

    fn map_song_stats_to_playlist_slots(
        song_stats_by_name: HashMap<String, SongStats>,
    ) -> BTreeMap<usize, String> {
        let ranked_songs = Self::rank_songs_for_setlist(song_stats_by_name);

        ranked_songs
            .into_iter()
            .enumerate()
            .map(|(index, (song_name, _))| (index + 1, song_name))
            .collect()
    }

    fn rank_songs_for_setlist(
        song_stats_by_name: HashMap<String, SongStats>,
    ) -> Vec<(String, SongStats)> {
        let mut ranked_songs: Vec<(String, SongStats)> = song_stats_by_name.into_iter().collect();

        ranked_songs.sort_by(|(_, a), (_, b)| {
            let a_bucket = Self::role_bucket_for_song(a);
            let b_bucket = Self::role_bucket_for_song(b);

            a_bucket
                .cmp(&b_bucket)
                .then_with(|| Self::compare_within_bucket(a, b, a_bucket))
                .then_with(|| b.total_plays.cmp(&a.total_plays))
                .then_with(|| a.song_name.cmp(&b.song_name))
        });

        ranked_songs
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
