use clap::{Parser, ValueEnum};
use crate::validator::artist_validator::ArtistValidator;

mod validator;

#[derive(Parser)]
struct Args {
    // artists to generate a setlist playlist for, separated by commas
    #[arg(short, long)]
    artists: String,

    #[arg(short, long)]
    // the name of the playlist to create
    playlist_name: Option<String>,

    // streaming service to use, either "spotify" or "youtube_music"
    #[arg(value_enum)]
    service: StreamingService,
}

#[derive(ValueEnum, Clone)]
enum StreamingService {
    Spotify,
    YouTubeMusic,
}

fn main() {
    let args = Args::parse();

    let _artists = match ArtistValidator::validate(&args) {
        Ok(artists) => artists,
        Err(err) => {
            eprintln!("Error validating artists: {}", err);
            std::process::exit(1);
        }
    };
}
