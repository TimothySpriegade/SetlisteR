use crate::Args;

pub struct ArtistValidator;

impl ArtistValidator {
    const MAX_ARTISTS: usize = 10;
    const MAX_ARTIST_NAME_LENGTH: usize = 100;

    pub fn validate(args: &Args) -> Result<Vec<String>, String> {
        let artists = Self::separate_artists(&args.artists);
        if artists.is_empty() {
            return Err("No artists provided. Please provide at least one artist.".to_string());
        }

        let normalized_artists: Vec<String> = artists
            .iter()
            .map(|artist| Self::normalize_whitespace(artist))
            .collect();

        let unique_artists = Self::remove_duplicates(normalized_artists);

        let filtered_artists = Self::filter_empty_artists(unique_artists);

        let valid_artists = Self::validate_amount_of_artists(filtered_artists)?;

        let artists = Self::validate_artist_names(valid_artists)?;

        Ok(artists)
    }

    fn normalize_whitespace(name: &str) -> String {
        name.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    fn remove_duplicates(artists: Vec<String>) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        artists
            .into_iter()
            .filter(|artist| seen.insert(artist.clone()))
            .collect()
    }

    fn validate_amount_of_artists(artists: Vec<String>) -> Result<Vec<String>, String> {
        if artists.len() > Self::MAX_ARTISTS {
            return Err(format!(
                "Too many artists provided. Please provide {} or fewer artists. You provided {} artists.",
                Self::MAX_ARTISTS,
                artists.len()
            ));
        }
        Ok(artists)
    }

    fn validate_artist_names(artists: Vec<String>) -> Result<Vec<String>, String> {
        let invalid_artists: Vec<String> = artists
            .iter()
            .filter(|artist| artist.len() > Self::MAX_ARTIST_NAME_LENGTH)
            .cloned()
            .collect();
        if !invalid_artists.is_empty() {
            return Err(format!(
                "The following artist names are too long (greater than {} characters): {}",
                Self::MAX_ARTIST_NAME_LENGTH,
                invalid_artists.join(", ")
            ));
        }

        Ok(artists)
    }

    fn separate_artists(artists: &str) -> Vec<String> {
        artists
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn filter_empty_artists(artists: Vec<String>) -> Vec<String> {
        artists
            .into_iter()
            .filter(|artist| !artist.is_empty())
            .collect()
    }
}
