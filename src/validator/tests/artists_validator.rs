use crate::validator::artist_validator::ArtistValidator;
use crate::Args;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_artists() {
        // Arrange
        let args = Args {
            artists: "Artist 1, Artist 2, Artist 3".to_string(),
            playlist_name: None,
            service: crate::StreamingService::Spotify,
        };

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        let artists = result.unwrap();
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
        assert_eq!(artists[2], "Artist 3");
    }

    #[test]
    fn test_validate_empty_artists() {
        // Arrange
        let args = Args {
            artists: "".to_string(),
            playlist_name: None,
            service: crate::StreamingService::Spotify,
        };

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "No artists provided. Please provide at least one artist.".to_string());
    }

    #[test]
    fn test_validate_too_many_artists() {
        // Arrange
        let args = Args {
            artists: "Artist 1, Artist 2, Artist 3, Artist 4, Artist 5, Artist 6, Artist 7, Artist 8, Artist 9, Artist 10, Artist 11".to_string(),
            playlist_name: None,
            service: crate::StreamingService::Spotify,
        };

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Too many artists provided. Please provide 10 or fewer artists. You provided 11 artists.".to_string());
    }

    #[test]
    fn test_validate_artist_name_too_long() {
        // Arrange
        let long_artist_name = "A".repeat(101);
        let args = Args {
            artists: long_artist_name.clone(),
            playlist_name: None,
            service: crate::StreamingService::Spotify,
        };

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), format!("The following artist names are too long (greater than 100 characters): {}", long_artist_name));
    }

    #[test]
    fn test_validate_artist_name_with_extra_whitespace() {
        // Arrange
        let args = Args {
            artists: "  Artist 1  ,   Artist 2   , Artist 3   ".to_string(),
            playlist_name: None,
            service: crate::StreamingService::Spotify,
        };

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        let artists = result.unwrap();
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
        assert_eq!(artists[2], "Artist 3");
    }

    #[test]
    fn test_validate_artist_name_with_duplicates() {
        // Arrange
        let args = Args {
            artists: "Artist 1, Artist 2, Artist 1, Artist 3".to_string(),
            playlist_name: None,
            service: crate::StreamingService::Spotify,
        };

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        let artists = result.unwrap();
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
        assert_eq!(artists[2], "Artist 3");
    }

    #[test]
    fn test_validate_artist_name_with_empty_names() {
        // Arrange
        let args = Args {
            artists: "Artist 1, , Artist 2, , Artist 3".to_string(),
            playlist_name: None,
            service: crate::StreamingService::Spotify,
        };

        // Act
        let result = ArtistValidator::validate(&args);

        // Assert
        let artists = result.unwrap();
        assert_eq!(artists.len(), 3);
        assert_eq!(artists[0], "Artist 1");
        assert_eq!(artists[1], "Artist 2");
        assert_eq!(artists[2], "Artist 3");
    }

}

