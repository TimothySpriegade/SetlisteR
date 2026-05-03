use crate::Args;
use crate::data::models::args::StreamingService;
use crate::secrets_manager::secrets_manager::KeyType;
use crate::validator::artist_validator::ArtistValidator;
use crate::validator::playlist_name_validator::PlaylistNameValidator;
use std::collections::HashMap;

pub struct ArgValidator {}

pub struct SanitizedArgs {
    pub artists: Vec<String>,
    pub playlist_name: String,
    pub service: StreamingService,
    pub page_depth: u16,
    pub secrets_by_type: HashMap<KeyType, String>,
    pub simple_mode: bool,
}
impl ArgValidator {
    pub fn validate(args: &Args) -> Result<SanitizedArgs, String> {
        let artists = ArtistValidator::validate(args)?;
        let playlist_name = PlaylistNameValidator::validate(args, &artists)?;
        let secrets_by_type = build_secrets_by_type(args);

        let simple_mode = args.simple_mode.unwrap_or(false);
        let page_depth = if simple_mode { 1 } else { args.page_depth };

        Ok(SanitizedArgs {
            artists,
            playlist_name,
            service: args.service.clone(),
            page_depth,
            secrets_by_type,
            simple_mode,
        })
    }
}

fn build_secrets_by_type(args: &Args) -> HashMap<KeyType, String> {
    let mut secrets_by_type = HashMap::new();

    if let Some(setlist_api_key) = &args.setlist_api_key {
        secrets_by_type.insert(KeyType::SetlistFmApiKey, setlist_api_key.clone());
    }

    secrets_by_type
}
