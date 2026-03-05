use crate::Args;
use crate::StreamingService;
use crate::validator::artist_validator::ArtistValidator;
use crate::validator::playlist_name_validator::PlaylistNameValidator;

pub struct ArgValidator {}

pub struct SanitizedArgs {
    pub artists: Vec<String>,
    pub playlist_name: String,
    pub service: StreamingService,
}
impl ArgValidator {
    pub fn validate(args: &Args) -> Result<SanitizedArgs, String> {
        let artists = ArtistValidator::validate(args)?;
        let playlist_name = PlaylistNameValidator::validate(args, &artists)?;

        Ok(SanitizedArgs {
            artists,
            playlist_name,
            service: args.service.clone(),
        })
    }
}
