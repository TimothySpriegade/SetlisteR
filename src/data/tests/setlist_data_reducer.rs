use crate::data::tests::setlist_data_reducer_mother::SetlistDataReducerMotherObject;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_orders_regular_songs_by_mean_position() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Song A".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song A", 2, 0, 0, 0, 3.0),
        );
        song_stats.insert(
            "Song B".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song B", 2, 0, 0, 0, 1.0),
        );
        song_stats.insert(
            "Song C".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song C", 2, 0, 0, 0, 2.0),
        );

        let reducer = SetlistDataReducerMotherObject::reducer(song_stats);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.get(&1), Some(&"Song B".to_string()));
        assert_eq!(songs.get(&2), Some(&"Song C".to_string()));
        assert_eq!(songs.get(&3), Some(&"Song A".to_string()));
    }

    #[test]
    fn test_reduce_prioritizes_exceptional_opener_regular_closer_then_encore_sections() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Opener Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Opener Song", 10, 9, 0, 0, 8.0),
        );
        song_stats.insert(
            "Regular Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Regular Song", 10, 1, 0, 0, 2.0),
        );
        song_stats.insert(
            "Closer Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Closer Song", 10, 0, 8, 0, 9.0),
        );
        song_stats.insert(
            "Encore Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Encore Song", 10, 0, 0, 8, 10.0),
        );

        let reducer = SetlistDataReducerMotherObject::reducer(song_stats);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.get(&1), Some(&"Opener Song".to_string()));
        assert_eq!(songs.get(&2), Some(&"Regular Song".to_string()));
        assert_eq!(songs.get(&3), Some(&"Closer Song".to_string()));
        assert_eq!(songs.get(&4), Some(&"Encore Song".to_string()));
    }

    #[test]
    fn test_reduce_does_not_treat_song_as_exceptional_when_total_plays_is_below_minimum() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Frequent Opener But Low Plays".to_string(),
            SetlistDataReducerMotherObject::song_stats(
                "Frequent Opener But Low Plays",
                2,
                2,
                0,
                0,
                5.0,
            ),
        );
        song_stats.insert(
            "Early Regular Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Early Regular Song", 2, 0, 0, 0, 1.0),
        );

        let reducer = SetlistDataReducerMotherObject::reducer(song_stats);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.get(&1), Some(&"Early Regular Song".to_string()));
        assert_eq!(
            songs.get(&2),
            Some(&"Frequent Opener But Low Plays".to_string())
        );
    }

    #[test]
    fn test_reduce_sorts_opener_bucket_by_rate_before_mean_position() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Higher Opener Rate".to_string(),
            SetlistDataReducerMotherObject::song_stats("Higher Opener Rate", 10, 9, 0, 0, 5.0),
        );
        song_stats.insert(
            "Lower Opener Rate".to_string(),
            SetlistDataReducerMotherObject::song_stats("Lower Opener Rate", 10, 8, 0, 0, 1.0),
        );

        let reducer = SetlistDataReducerMotherObject::reducer(song_stats);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.get(&1), Some(&"Higher Opener Rate".to_string()));
        assert_eq!(songs.get(&2), Some(&"Lower Opener Rate".to_string()));
    }

    #[test]
    fn test_reduce_limits_playlist_length_to_rounded_up_average_setlist_size() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Song A".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song A", 10, 0, 0, 0, 1.0),
        );
        song_stats.insert(
            "Song B".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song B", 10, 0, 0, 0, 2.0),
        );
        song_stats.insert(
            "Song C".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song C", 10, 0, 0, 0, 3.0),
        );

        let reducer = SetlistDataReducerMotherObject::reducer_with_average(song_stats, 2.1);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.len(), 3);
        assert_eq!(songs.get(&1), Some(&"Song A".to_string()));
        assert_eq!(songs.get(&2), Some(&"Song B".to_string()));
        assert_eq!(songs.get(&3), Some(&"Song C".to_string()));
    }

    #[test]
    fn test_reduce_applies_offset_to_positive_average_setlist_size() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Song A".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song A", 70, 0, 0, 0, 1.0),
        );
        song_stats.insert(
            "Song B".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song B", 60, 0, 0, 0, 2.0),
        );
        song_stats.insert(
            "Song C".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song C", 50, 0, 0, 0, 3.0),
        );
        song_stats.insert(
            "Song D".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song D", 40, 0, 0, 0, 4.0),
        );
        song_stats.insert(
            "Song E".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song E", 30, 0, 0, 0, 5.0),
        );
        song_stats.insert(
            "Song F".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song F", 20, 0, 0, 0, 6.0),
        );
        song_stats.insert(
            "Song G".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song G", 10, 0, 0, 0, 7.0),
        );

        let reducer = SetlistDataReducerMotherObject::reducer_with_average(song_stats, 2.1);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.len(), 6);
        assert_eq!(songs.get(&1), Some(&"Song A".to_string()));
        assert_eq!(songs.get(&2), Some(&"Song B".to_string()));
        assert_eq!(songs.get(&3), Some(&"Song C".to_string()));
        assert_eq!(songs.get(&4), Some(&"Song D".to_string()));
        assert_eq!(songs.get(&5), Some(&"Song E".to_string()));
        assert_eq!(songs.get(&6), Some(&"Song F".to_string()));
        assert!(songs.values().all(|song| song != "Song G"));
    }

    #[test]
    fn test_reduce_limits_playlist_length_when_average_is_lower_than_song_count() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Song A".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song A", 10, 0, 0, 0, 1.0),
        );
        song_stats.insert(
            "Song B".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song B", 10, 0, 0, 0, 2.0),
        );
        song_stats.insert(
            "Song C".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song C", 10, 0, 0, 0, 3.0),
        );

        let reducer = SetlistDataReducerMotherObject::reducer_with_average(song_stats, 1.2);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.len(), 3);
        assert_eq!(songs.get(&1), Some(&"Song A".to_string()));
        assert_eq!(songs.get(&2), Some(&"Song B".to_string()));
        assert_eq!(songs.get(&3), Some(&"Song C".to_string()));
    }

    #[test]
    fn test_reduce_selects_most_played_songs_before_position_ordering() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Least Played Early Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Least Played Early Song", 1, 0, 0, 0, 1.0),
        );
        song_stats.insert(
            "Most Played Later Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Most Played Later Song", 100, 0, 0, 0, 3.0),
        );
        song_stats.insert(
            "Second Most Played Mid Song".to_string(),
            SetlistDataReducerMotherObject::song_stats(
                "Second Most Played Mid Song",
                90,
                0,
                0,
                0,
                2.0,
            ),
        );
        song_stats.insert(
            "Third Most Played Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Third Most Played Song", 80, 0, 0, 0, 1.5),
        );
        song_stats.insert(
            "Fourth Most Played Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Fourth Most Played Song", 70, 0, 0, 0, 2.5),
        );
        song_stats.insert(
            "Fifth Most Played Song".to_string(),
            SetlistDataReducerMotherObject::song_stats("Fifth Most Played Song", 60, 0, 0, 0, 4.0),
        );

        let reducer = SetlistDataReducerMotherObject::reducer_with_average(song_stats, 1.0);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.len(), 4);
        assert_eq!(songs.get(&1), Some(&"Third Most Played Song".to_string()));
        assert_eq!(
            songs.get(&2),
            Some(&"Second Most Played Mid Song".to_string())
        );
        assert_eq!(songs.get(&3), Some(&"Fourth Most Played Song".to_string()));
        assert_eq!(songs.get(&4), Some(&"Most Played Later Song".to_string()));
        assert!(songs.values().all(|song| song != "Least Played Early Song"));
        assert!(songs.values().all(|song| song != "Fifth Most Played Song"));
    }

    #[test]
    fn test_reduce_returns_empty_playlist_when_average_setlist_size_is_zero() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Song A".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song A", 10, 0, 0, 0, 1.0),
        );
        let reducer = SetlistDataReducerMotherObject::reducer_with_average(song_stats, 0.0);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.len(), 0);
    }

    #[test]
    fn test_reduce_returns_empty_playlist_when_average_setlist_size_is_negative() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Song A".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song A", 10, 0, 0, 0, 1.0),
        );
        let reducer = SetlistDataReducerMotherObject::reducer_with_average(song_stats, -1.0);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.len(), 0);
    }

    #[test]
    fn test_reduce_returns_empty_playlist_when_average_setlist_size_is_nan() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Song A".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song A", 10, 0, 0, 0, 1.0),
        );
        let reducer = SetlistDataReducerMotherObject::reducer_with_average(song_stats, f32::NAN);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.len(), 0);
    }

    #[test]
    fn test_reduce_returns_empty_playlist_when_average_setlist_size_is_infinite() {
        // Arrange
        let mut song_stats = HashMap::new();
        song_stats.insert(
            "Song A".to_string(),
            SetlistDataReducerMotherObject::song_stats("Song A", 10, 0, 0, 0, 1.0),
        );
        let reducer =
            SetlistDataReducerMotherObject::reducer_with_average(song_stats, f32::INFINITY);

        // Act
        let playlist_data = reducer.reduce();

        // Assert
        let songs = &playlist_data.artist_playlists[0].songs_by_position;
        assert_eq!(songs.len(), 0);
    }
}
