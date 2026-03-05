use clap::{Parser, ValueEnum};
use crate::validator::arg_validator::ArgValidator;

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

    let _sanitized_args = match ArgValidator::validate(&args) {
        Ok(validated_args) => validated_args,
        Err(err) => {
            eprintln!("Error validating arguments: {}", err);
            std::process::exit(1);
        }
    };
}
