use crate::validator::arg_validator::ArgValidator;
use clap::{Parser, ValueEnum};

pub mod api;
mod validator;
pub mod data;

#[derive(Parser)]
struct Args {
    // artists to generate a setlist playlist for, separated by commas
    #[arg(short, long)]
    artists: String,

    #[arg(short, long)]
    // the name of the playlist to create
    playlist_name: Option<String>,

    // streaming service to use, either "spotify" or "youtube_music" currently supported
    #[arg(short, long, value_enum)]
    service: StreamingService,

    // how many pages of setlist data to fetch from the setlist.fm API
    #[arg(long, default_value_t = 1)]
    page_depth: u16,
}

#[derive(ValueEnum, Clone)]
enum StreamingService {
    Spotify,
    YouTubeMusic,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    dotenvy::dotenv().ok();
    let api_key = std::env::var("SETLIST_FM_API_KEY").expect("SETLIST_FM_API_KEY must be set");

    let sanitized_args = match ArgValidator::validate(&args) {
        Ok(validated_args) => validated_args,
        Err(err) => {
            eprintln!("Error validating arguments: {}", err);
            std::process::exit(1);
        }
    };

    let setlist_fm_client = api::setlist_fm::SetlistFmClient::new(api_key);
    for artist in &sanitized_args.artists {
        let setlist_fm_setlist_data = setlist_fm_client.get_setlist_by_artist(artist, args.page_depth).await;
        println!("{:#?}", setlist_fm_setlist_data);
    }
}
