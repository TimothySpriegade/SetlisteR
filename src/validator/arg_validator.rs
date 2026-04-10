use crate::Args;
use crate::StreamingService;
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
    pub secret_hashmap: HashMap<KeyType, String>,
}
impl ArgValidator {
    pub fn validate(args: &Args) -> Result<SanitizedArgs, String> {
        let artists = ArtistValidator::validate(args)?;
        let playlist_name = PlaylistNameValidator::validate(args, &artists)?;

        let secret_hashmap = build_secret_hashmap(args);

        Ok(SanitizedArgs {
            artists,
            playlist_name,
            service: args.service.clone(),
            page_depth: args.page_depth,
            secret_hashmap: secret_hashmap,
        })
    }
}

fn build_secret_hashmap(args: &Args) -> HashMap<KeyType, String> {
    let mut secret_hashmap = HashMap::new();

    if let Some(setlist_api_key) = &args.setlist_api_key {
        secret_hashmap.insert(KeyType::SetlistFmApiKey, setlist_api_key.clone());
    }

    secret_hashmap
}
