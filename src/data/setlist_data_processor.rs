use crate::data::models::setlistfm_response::{Set, Setlist, Song};
use crate::data::models::song_stats::SongStats;
use chrono::NaiveDate;
use std::collections::HashMap;

pub struct SetlistDataProcessor;

impl SetlistDataProcessor {
    pub fn reduce_to_song_stats(setlists: &[Setlist]) -> HashMap<String, SongStats> {
        let mut song_stats_by_name: HashMap<String, SongStats> = HashMap::new();

        for setlist in setlists {
            Self::process_show(setlist, &mut song_stats_by_name);
        }

        Self::populate_mean_positions(&mut song_stats_by_name);

        song_stats_by_name
    }

    pub fn average_songs_per_setlist(setlists: &[Setlist]) -> f32 {
        if setlists.is_empty() {
            return 0.0;
        }

        let total_songs: usize = setlists
            .iter()
            .map(Self::count_non_tape_songs_in_show)
            .sum();
        total_songs as f32 / setlists.len() as f32
    }

    fn process_show(setlist: &Setlist, song_stats_by_name: &mut HashMap<String, SongStats>) {
        let mut show_song_position = 0;
        let event_date = setlist.event_date.as_str();
        let show_date = Self::parse_event_date(event_date);

        if setlist.sets.set.is_empty() {
            return;
        }

        for set in &setlist.sets.set {
            Self::process_set(
                set,
                &mut show_song_position,
                song_stats_by_name,
                event_date,
                show_date.as_ref(),
            );
        }
    }

    fn process_set(
        set: &Set,
        show_song_position: &mut usize,
        song_stats_by_name: &mut HashMap<String, SongStats>,
        event_date: &str,
        show_date: Option<&NaiveDate>,
    ) {
        let is_encore = set.encore.is_some();
        let songs_in_set_count = set.song.len();

        for (song_index_in_set, song) in set.song.iter().enumerate() {
            if song.tape.unwrap_or(false) {
                continue;
            }

            *show_song_position += 1;

            let song_name = Self::resolve_song_name(song);
            let is_opener = *show_song_position == 1;
            let is_set_closer = song_index_in_set == songs_in_set_count - 1;

            Self::update_song_stats(
                song_stats_by_name,
                song_name,
                *show_song_position,
                is_opener,
                is_encore,
                is_set_closer,
                event_date,
                show_date,
            );
        }
    }

    fn update_song_stats(
        song_stats_by_name: &mut HashMap<String, SongStats>,
        song_name: String,
        song_position: usize,
        is_opener: bool,
        is_encore: bool,
        is_set_closer: bool,
        event_date: &str,
        show_date: Option<&NaiveDate>,
    ) {
        let stats = song_stats_by_name
            .entry(song_name.clone())
            .or_insert_with(|| SongStats {
                song_name: song_name.clone(),
                ..Default::default()
            });

        stats.total_plays += 1;
        stats.positions_played.push(song_position);

        if is_opener {
            stats.opener_count += 1;
        }

        if is_encore {
            stats.encore_count += 1;
        }

        if is_set_closer && !is_encore {
            stats.closer_count += 1;
        }

        if let Some(show_date) = show_date {
            let update_last_played = match Self::parse_event_date(stats.last_played.as_str()) {
                Some(existing_date) => *show_date > existing_date,
                None => true,
            };

            if update_last_played {
                stats.last_played = event_date.to_string();
            }
        }
    }

    fn populate_mean_positions(song_stats_by_name: &mut HashMap<String, SongStats>) {
        for stats in song_stats_by_name.values_mut() {
            if !stats.positions_played.is_empty() {
                let total_positions: usize = stats.positions_played.iter().sum();
                let count = stats.positions_played.len() as f32;
                stats
                    .mean_positions_played
                    .push(total_positions as f32 / count);
            }
        }
    }

    fn count_non_tape_songs_in_show(show: &Setlist) -> usize {
        show.sets
            .set
            .iter()
            .map(|set| {
                set.song
                    .iter()
                    .filter(|song| !song.tape.unwrap_or(false))
                    .count()
            })
            .sum()
    }

    fn resolve_song_name(song: &Song) -> String {
        match &song.name {
            Some(name) if !name.trim().is_empty() => name.clone(),
            _ => match &song.info {
                Some(info) if !info.trim().is_empty() => info.clone(),
                _ => "Unknown Song".to_string(),
            },
        }
    }

    fn parse_event_date(event_date: &str) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(event_date, "%d-%m-%Y").ok()
    }
}
