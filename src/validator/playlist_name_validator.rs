pub struct PlaylistNameValidator {}

impl PlaylistNameValidator {
    const MAX_PLAYLIST_NAME_LENGTH: usize = 100;

    pub fn validate(args: &crate::Args, sanitized_artist: &Vec<String>) -> Result<String, String> {
        if let Some(playlist_name) = &args.playlist_name {
            let normalized_name = Self::normalize_whitespace(playlist_name);

            let collapsed_name = Self::collapse_whitespace(&normalized_name);

            let stripped_name = Self::strip_non_unicode(&collapsed_name);

            Self::validate_length(&stripped_name)?;

            Ok(stripped_name)
        } else {
            Ok(Self::generate_default_name(&sanitized_artist))
        }
    }

    fn generate_default_name(artists: &[String]) -> String {
        let playlist_name = if artists.len() == 1 {
            format!("{} Setlist", artists[0])
        } else {
            format!(
                "{} and {} Setlist",
                artists[..artists.len() - 1].join(", "),
                artists.last().unwrap()
            )
        };

        match Self::validate_length(&playlist_name) {
            Ok(_) => playlist_name,
            Err(_) => {
                print!(
                    "Generated playlist name is too long. Truncating to {} characters.",
                    Self::MAX_PLAYLIST_NAME_LENGTH
                );
                playlist_name
                    .chars()
                    .take(Self::MAX_PLAYLIST_NAME_LENGTH)
                    .collect()
            }
        }
    }

    fn normalize_whitespace(name: &str) -> String {
        name.trim().to_string()
    }

    fn collapse_whitespace(name: &str) -> String {
        name.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    fn strip_non_unicode(name: &str) -> String {
        name.chars().filter(|c| c.is_ascii()).collect()
    }

    fn validate_length(name: &str) -> Result<(), String> {
        if name.len() > Self::MAX_PLAYLIST_NAME_LENGTH {
            return Err(format!(
                "Playlist name is too long (greater than {} characters). Please provide a shorter playlist name.",
                Self::MAX_PLAYLIST_NAME_LENGTH
            ));
        }
        Ok(())
    }
}
