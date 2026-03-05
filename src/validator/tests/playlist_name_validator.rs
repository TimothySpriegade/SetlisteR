use crate::validator::playlist_name_validator::PlaylistNameValidator;
use crate::validator::tests::args_mother::ArgsMotherObject;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_playlist_name() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_playlist_name(Some("My Playlist"))
            .build();
        let artists = vec!["Artist 1".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), "My Playlist");
    }

    #[test]
    fn test_validate_playlist_name_too_long() {
        // Arrange
        let long_name = "A".repeat(101);
        let args = ArgsMotherObject::default()
            .with_playlist_name(Some(&long_name))
            .build();
        let artists = vec!["Artist 1".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Playlist name is too long (greater than 100 characters). Please provide a shorter playlist name.".to_string()
        );
    }

    #[test]
    fn test_validate_playlist_name_exactly_100_characters() {
        // Arrange
        let name = "A".repeat(100);
        let args = ArgsMotherObject::default()
            .with_playlist_name(Some(&name))
            .build();
        let artists = vec!["Artist 1".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), name);
    }

    #[test]
    fn test_validate_playlist_name_with_leading_trailing_whitespace() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_playlist_name(Some("  My Playlist  "))
            .build();
        let artists = vec!["Artist 1".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), "My Playlist");
    }

    #[test]
    fn test_validate_playlist_name_with_extra_inner_whitespace() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_playlist_name(Some("My   Playlist"))
            .build();
        let artists = vec!["Artist 1".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), "My Playlist");
    }

    #[test]
    fn test_validate_playlist_name_strips_non_ascii() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_playlist_name(Some("My Playlïst"))
            .build();
        let artists = vec!["Artist 1".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), "My Playlst");
    }

    #[test]
    fn test_validate_playlist_name_entirely_non_ascii_becomes_empty() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_playlist_name(Some("ïïï"))
            .build();
        let artists = vec!["Artist 1".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_validate_generates_default_name_for_single_artist() {
        // Arrange
        let args = ArgsMotherObject::default().build();
        let artists = vec!["Artist 1".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), "Artist 1 Setlist");
    }

    #[test]
    fn test_validate_generates_default_name_for_two_artists() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, Artist 2")
            .build();
        let artists = vec!["Artist 1".to_string(), "Artist 2".to_string()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), "Artist 1 and Artist 2 Setlist");
    }

    #[test]
    fn test_validate_generates_default_name_for_multiple_artists() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, Artist 2, Artist 3")
            .build();
        let artists = vec![
            "Artist 1".to_string(),
            "Artist 2".to_string(),
            "Artist 3".to_string(),
        ];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), "Artist 1, Artist 2 and Artist 3 Setlist");
    }

    #[test]
    fn test_validate_generates_default_name_truncated_when_too_long() {
        // Arrange
        let long_artist_name = "A".repeat(100);
        let args = ArgsMotherObject::default()
            .with_artists(&long_artist_name)
            .build();
        let artists = vec![long_artist_name];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap().len(), 100);
    }

    #[test]
    fn test_validate_generates_default_name_truncated_content_is_correct() {
        // Arrange
        let long_artist_name = "A".repeat(100);
        let args = ArgsMotherObject::default()
            .with_artists(&long_artist_name)
            .build();
        let artists = vec![long_artist_name.clone()];
        let expected = format!("{} Setlist", long_artist_name)
            .chars()
            .take(100)
            .collect::<String>();

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_validate_generates_default_name_multiple_long_artists_truncated() {
        // Arrange
        let long_artist_name = "A".repeat(50);
        let args = ArgsMotherObject::default()
            .with_artists(&format!("{}, {}", long_artist_name, long_artist_name))
            .build();
        let artists = vec![long_artist_name.clone(), long_artist_name.clone()];

        // Act
        let result = PlaylistNameValidator::validate(&args, &artists);

        // Assert
        assert_eq!(result.unwrap().len(), 100);
    }
}
