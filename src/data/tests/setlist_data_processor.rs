use crate::data::setlist_data_processor::SetlistDataProcessor;
use crate::data::tests::setlist_mother::SetlistMotherObject;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_to_song_stats_tracks_positions_openers_closers_and_encores() {
        // Arrange
        let show = SetlistMotherObject::show(
            "21-09-2023",
            vec![
                SetlistMotherObject::set(
                    None,
                    vec![
                        SetlistMotherObject::song(Some("Song A"), None, Some(false)),
                        SetlistMotherObject::song(Some("Tape Intro"), None, Some(true)),
                        SetlistMotherObject::song(Some("Song C"), None, Some(false)),
                    ],
                ),
                SetlistMotherObject::set(
                    Some(1),
                    vec![SetlistMotherObject::song(Some("Song A"), None, Some(false))],
                ),
            ],
        );

        // Act
        let stats_map = SetlistDataProcessor::reduce_to_song_stats(&[show]);

        // Assert
        let song_a = stats_map.get("Song A").unwrap();
        assert_eq!(song_a.total_plays, 2);
        assert_eq!(song_a.positions_played, vec![1, 3]);
        assert_eq!(song_a.opener_count, 1);
        assert_eq!(song_a.encore_count, 1);
        assert_eq!(song_a.closer_count, 0);
        assert_eq!(song_a.last_played, "21-09-2023");
        assert_eq!(song_a.mean_positions_played.last(), Some(&2.0));

        let song_c = stats_map.get("Song C").unwrap();
        assert_eq!(song_c.total_plays, 1);
        assert_eq!(song_c.positions_played, vec![2]);
        assert_eq!(song_c.opener_count, 0);
        assert_eq!(song_c.encore_count, 0);
        assert_eq!(song_c.closer_count, 1);
        assert_eq!(song_c.last_played, "21-09-2023");
        assert_eq!(song_c.mean_positions_played.last(), Some(&2.0));
    }

    #[test]
    fn test_reduce_to_song_stats_falls_back_to_info_then_unknown_song() {
        // Arrange
        let show = SetlistMotherObject::show(
            "21-09-2023",
            vec![SetlistMotherObject::set(
                None,
                vec![
                    SetlistMotherObject::song(None, Some("Jam Session"), Some(false)),
                    SetlistMotherObject::song(None, None, Some(false)),
                ],
            )],
        );

        // Act
        let stats_map = SetlistDataProcessor::reduce_to_song_stats(&[show]);

        // Assert
        assert!(stats_map.contains_key("Jam Session"));
        assert!(stats_map.contains_key("Unknown Song"));
    }

    #[test]
    fn test_reduce_to_song_stats_keeps_newer_last_played_when_input_order_is_unsorted() {
        // Arrange
        let newer_show = SetlistMotherObject::show(
            "21-09-2023",
            vec![SetlistMotherObject::set(
                None,
                vec![SetlistMotherObject::song(Some("Song A"), None, Some(false))],
            )],
        );
        let older_show = SetlistMotherObject::show(
            "05-01-2023",
            vec![SetlistMotherObject::set(
                None,
                vec![SetlistMotherObject::song(Some("Song A"), None, Some(false))],
            )],
        );

        // Act
        let stats_map = SetlistDataProcessor::reduce_to_song_stats(&[newer_show, older_show]);

        // Assert
        let song_a = stats_map.get("Song A").unwrap();
        assert_eq!(song_a.last_played, "21-09-2023");
    }

    #[test]
    fn test_reduce_to_song_stats_ignores_invalid_event_date_for_last_played() {
        // Arrange
        let invalid_date_show = SetlistMotherObject::show(
            "2023-09-21",
            vec![SetlistMotherObject::set(
                None,
                vec![SetlistMotherObject::song(Some("Song A"), None, Some(false))],
            )],
        );

        // Act
        let stats_map = SetlistDataProcessor::reduce_to_song_stats(&[invalid_date_show]);

        // Assert
        let song_a = stats_map.get("Song A").unwrap();
        assert_eq!(song_a.last_played, "");
    }

    #[test]
    fn test_reduce_to_song_stats_calculates_expected_mean_positions() {
        // Arrange
        let show = SetlistMotherObject::show(
            "21-09-2023",
            vec![SetlistMotherObject::set(
                None,
                vec![
                    SetlistMotherObject::song(Some("Song A"), None, Some(false)),
                    SetlistMotherObject::song(Some("Song B"), None, Some(false)),
                    SetlistMotherObject::song(Some("Song A"), None, Some(false)),
                ],
            )],
        );
        // Act
        let stats_map = SetlistDataProcessor::reduce_to_song_stats(&[show]);

        // Assert
        let song_a = stats_map.get("Song A").unwrap();
        assert_eq!(song_a.positions_played, vec![1, 3]);
        assert_eq!(song_a.mean_positions_played.last(), Some(&2.0));

        let song_b = stats_map.get("Song B").unwrap();
        assert_eq!(song_b.positions_played, vec![2]);
        assert_eq!(song_b.mean_positions_played.last(), Some(&2.0));
    }

    #[test]
    fn test_average_songs_per_setlist_calculates_expected_average() {
        // Arrange
        let show_with_two_songs = SetlistMotherObject::show(
            "21-09-2023",
            vec![SetlistMotherObject::set(
                None,
                vec![
                    SetlistMotherObject::song(Some("Song A"), None, Some(false)),
                    SetlistMotherObject::song(Some("Tape Intro"), None, Some(true)),
                    SetlistMotherObject::song(Some("Song B"), None, Some(false)),
                ],
            )],
        );
        let show_with_four_songs = SetlistMotherObject::show(
            "22-09-2023",
            vec![
                SetlistMotherObject::set(
                    None,
                    vec![
                        SetlistMotherObject::song(Some("Song C"), None, Some(false)),
                        SetlistMotherObject::song(Some("Song D"), None, Some(false)),
                    ],
                ),
                SetlistMotherObject::set(
                    Some(1),
                    vec![
                        SetlistMotherObject::song(Some("Song E"), None, Some(false)),
                        SetlistMotherObject::song(Some("Song F"), None, Some(false)),
                    ],
                ),
            ],
        );

        // Act
        let average = SetlistDataProcessor::average_songs_per_setlist(&[
            show_with_two_songs,
            show_with_four_songs,
        ]);

        // Assert
        assert_eq!(average, 3.0);
    }

    #[test]
    fn test_average_songs_per_setlist_returns_zero_for_empty_input() {
        // Act
        let average = SetlistDataProcessor::average_songs_per_setlist(&[]);

        // Assert
        assert_eq!(average, 0.0);
    }
}
