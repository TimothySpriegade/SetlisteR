use crate::validator::artist_validator::ArtistValidator;
use crate::validator::tests::args_mother::ArgsMotherObject;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_artists() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, Artist 2, Artist 3")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
        assert_eq!(artists[2], "Artist 3");
    }

    #[test]
    fn test_validate_empty_artists() {
        // Arrange
        let args = ArgsMotherObject::default().with_artists("").build();

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "No artists provided. Please provide at least one artist.".to_string()
        );
    }

    #[test]
    fn test_validate_too_many_artists() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, Artist 2, Artist 3, Artist 4, Artist 5, Artist 6, Artist 7, Artist 8, Artist 9, Artist 10, Artist 11")
            .build();

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Too many artists provided. Please provide 10 or fewer artists. You provided 11 artists.".to_string()
        );
    }

    #[test]
    fn test_validate_artist_name_too_long() {
        // Arrange
        let long_artist_name = "A".repeat(101);
        let args = ArgsMotherObject::default()
            .with_artists(&long_artist_name)
            .build();

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            format!(
                "The following artist names are too long (greater than 100 characters): {}",
                long_artist_name
            )
        );
    }

    #[test]
    fn test_validate_artist_name_with_extra_whitespace() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("  Artist 1  ,   Artist 2   , Artist 3   ")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
        assert_eq!(artists[2], "Artist 3");
    }

    #[test]
    fn test_validate_artist_name_with_duplicates() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, Artist 2, Artist 1, Artist 3")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
        assert_eq!(artists[2], "Artist 3");
    }

    #[test]
    fn test_validate_artist_name_with_empty_names() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, , Artist 2, , Artist 3")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
        assert_eq!(artists[2], "Artist 3");
    }

    #[test]
    fn test_validate_artist_name_with_non_unicode_characters() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, Artïst 2, Artïst 3")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artst 2");
        assert_eq!(artists[2], "Artst 3");
    }

    #[test]
    fn test_validate_single_artist() {
        // Arrange
        let args = ArgsMotherObject::default().with_artists("Artist 1").build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 1);
        assert_eq!(artists[0], "Artist 1");
    }

    #[test]
    fn test_validate_exactly_ten_artists() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, Artist 2, Artist 3, Artist 4, Artist 5, Artist 6, Artist 7, Artist 8, Artist 9, Artist 10")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 10);
    }

    #[test]
    fn test_validate_artist_name_exactly_100_characters() {
        // Arrange
        let artist_name = "A".repeat(100);
        let args = ArgsMotherObject::default()
            .with_artists(&artist_name)
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 1);
        assert_eq!(artists[0], artist_name);
    }

    #[test]
    fn test_validate_case_sensitive_duplicates_not_removed() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("artist 1, Artist 1")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 2);
    }

    #[test]
    fn test_validate_artist_entirely_non_ascii_becomes_empty() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1, ïïï, Artist 3")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 2);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 3");
    }

    #[test]
    fn test_validate_non_ascii_stripping_creates_duplicates() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artïst, Artüst")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 1);
        assert_eq!(artists[0], "Artst");
    }

    #[test]
    fn test_validate_whitespace_only_artist_is_filtered() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_artists("Artist 1,    , Artist 2")
            .build();

        // Act
        let artists = ArtistValidator::validate(&args).unwrap();

        // Assert
        assert_eq!(artists.len(), 2);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
    }

    #[test]
    fn test_validate_only_non_ascii_artists_returns_error() {
        // Arrange
        let args = ArgsMotherObject::default().with_artists("ïïï, üüü").build();

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "No valid artists provided after processing. Please provide at least one valid artist."
                .to_string()
        );
    }

    #[test]
    fn test_default_page_depth_is_one() {
        // Arrange
        let args = ArgsMotherObject::default().build();

        // Assert
        assert_eq!(args.page_depth, 1);
    }

    #[test]
    fn test_page_depth_can_be_overridden() {
        // Arrange
        let args = ArgsMotherObject::default().with_page_depth(5).build();

        // Assert
        assert_eq!(args.page_depth, 5);
    }

    #[test]
    fn test_page_depth_minimum_value() {
        // Arrange
        let args = ArgsMotherObject::default().with_page_depth(1).build();

        // Assert
        assert_eq!(args.page_depth, 1);
    }

    #[test]
    fn test_page_depth_maximum_u16_value() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_page_depth(u16::MAX)
            .build();

        // Assert
        assert_eq!(args.page_depth, u16::MAX);
    }
}
