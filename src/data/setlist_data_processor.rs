use crate::data::models::setlistfm_response_models::{Set, Setlist, Song};
use crate::data::models::song_stats::SongStats;
use chrono::NaiveDate;
use std::collections::HashMap;

pub struct SetlistDataProcessor;

impl SetlistDataProcessor {
    pub fn reduce_to_song_stats(setlists: &[Setlist]) -> HashMap<String, SongStats> {
        let mut stats_map: HashMap<String, SongStats> = HashMap::new();

        for show in setlists {
            Self::process_show(show, &mut stats_map);
        }
        
        Self::calculate_mean_positions(&mut stats_map);

        stats_map
    }

    pub fn average_songs_per_setlist(setlists: &[Setlist]) -> f32 {
        if setlists.is_empty() {
            return 0.0;
        }

        let total_songs: usize = setlists.iter().map(Self::count_non_tape_songs_in_show).sum();
        total_songs as f32 / setlists.len() as f32
    }

    fn process_show(show: &Setlist, stats_map: &mut HashMap<String, SongStats>) {
        let mut global_song_index = 0;
        let event_date = show.event_date.as_str();
        let parsed_event_date = Self::parse_event_date(event_date);

        if show.sets.set.is_empty() {
            return;
        }

        for set in &show.sets.set {
            Self::process_set(
                set,
                &mut global_song_index,
                stats_map,
                event_date,
                parsed_event_date.as_ref(),
            );
        }
    }

    fn process_set(
        set: &Set,
        global_song_index: &mut usize,
        stats_map: &mut HashMap<String, SongStats>,
        event_date: &str,
        parsed_event_date: Option<&NaiveDate>,
    ) {
        let is_encore = set.encore.is_some();
        let num_songs_in_set = set.song.len();

        for (i, song) in set.song.iter().enumerate() {
            if song.tape.unwrap_or(false) {
                continue;
            }

            *global_song_index += 1;

            let song_name = Self::resolve_song_name(song);
            let is_opener = *global_song_index == 1;
            let is_set_closer = i == num_songs_in_set - 1;

            Self::update_song_stats(
                stats_map,
                song_name,
                *global_song_index,
                is_opener,
                is_encore,
                is_set_closer,
                event_date,
                parsed_event_date,
            );
        }
    }

    fn update_song_stats(
        stats_map: &mut HashMap<String, SongStats>,
        song_name: String,
        song_position: usize,
        is_opener: bool,
        is_encore: bool,
        is_set_closer: bool,
        event_date: &str,
        parsed_event_date: Option<&NaiveDate>,
    ) {
        let stats = stats_map
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

        if let Some(show_date) = parsed_event_date {
            let update_last_played = match Self::parse_event_date(stats.last_played.as_str()) {
                Some(existing_date) => *show_date > existing_date,
                None => true,
            };

            if update_last_played {
                stats.last_played = event_date.to_string();
            }
        }
    }

    fn calculate_mean_positions(
        stats_map: &mut HashMap<String, SongStats>,
    ) -> HashMap<String, SongStats> {
        for stats in stats_map.values_mut() {
            if !stats.positions_played.is_empty() {
                let total_positions: usize = stats.positions_played.iter().sum();
                let count = stats.positions_played.len() as f32;
                stats
                    .mean_positions_played
                    .push(total_positions as f32 / count);
            }
        }

        stats_map.clone()
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
