use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    // artists to generate a setlist playlist for, separated by commas
    #[arg(short, long)]
    pub(crate) artists: String,

    #[arg(short, long)]
    // the name of the playlist to create
    pub(crate) playlist_name: Option<String>,

    // streaming service to use, either "spotify" or "youtube_music" currently supported
    #[arg(short, long, value_enum)]
    pub(crate) service: StreamingService,

    // how many pages of setlist data to fetch from the setlist.fm API
    #[arg(long, default_value_t = 1)]
    pub(crate) page_depth: u16,

    /// Store the setlist.fm API key in the system keyring
    #[arg(long)]
    pub(crate) setlist_api_key: Option<String>,
}

#[derive(ValueEnum, Clone)]
pub(crate) enum StreamingService {
    Spotify,
    YouTubeMusic,
}
