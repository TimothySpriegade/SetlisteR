use crate::data::models::args::StreamingService;
use crate::data::models::meta_data::CollectedData;
use crate::data::models::playlist_data::{ArtistSongData, PlaylistData};
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
    collected_data: CollectedData,
}

impl SetlistDataReducer {
    const MIN_ROLE_PLAYS: u32 = 3;
    const EXCEPTIONAL_ROLE_RATE: f32 = 0.60;

    pub fn new(
        playlist_name: String,
        streaming_service: StreamingService,
        collected_data: CollectedData,
    ) -> Self {
        Self {
            playlist_name,
            streaming_service,
            collected_data,
        }
    }

    pub fn reduce(&self) -> PlaylistData {
        let mut playlist_data = PlaylistData {
            playlist_name: self.playlist_name.clone(),
            platforms: vec![self.streaming_service.clone()],
            artist_song_data: Vec::new(),
        };

        let mut artist_song_data = Vec::new();

        for data in &self.collected_data.collected_meta_data {
            let artist = data.artist_name.clone();
            let setlist_data = Self::reduce_to_playlist_data(data.song_stats.clone());

            artist_song_data.push(ArtistSongData {
                artist,
                songs: setlist_data,
            });
        }
        playlist_data.artist_song_data = artist_song_data;
        playlist_data
    }

    fn reduce_to_playlist_data(song_data: HashMap<String, SongStats>) -> BTreeMap<usize, String> {
        let sorted_song_data = Self::sort_setlist_by_mean_position(song_data);

        sorted_song_data
            .into_iter()
            .enumerate()
            .map(|(index, (song_name, _))| (index + 1, song_name))
            .collect()
    }

    fn sort_setlist_by_mean_position(
        song_data: HashMap<String, SongStats>,
    ) -> Vec<(String, SongStats)> {
        let mut sorted_song_data: Vec<(String, SongStats)> = song_data.into_iter().collect();

        sorted_song_data.sort_by(|(_, a), (_, b)| {
            let a_bucket = Self::role_bucket(a);
            let b_bucket = Self::role_bucket(b);

            a_bucket
                .cmp(&b_bucket)
                .then_with(|| Self::compare_within_bucket(a, b, a_bucket))
                .then_with(|| b.total_plays.cmp(&a.total_plays))
                .then_with(|| a.song_name.cmp(&b.song_name))
        });

        sorted_song_data
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

    fn role_bucket(stats: &SongStats) -> RoleBucket {
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
