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
            SetlistDataReducerMotherObject::song_stats("Frequent Opener But Low Plays", 2, 2, 0, 0, 5.0),
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
        assert_eq!(songs.get(&2), Some(&"Frequent Opener But Low Plays".to_string()));
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

}
