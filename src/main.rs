use crate::validator::arg_validator::ArgValidator;
use clap::{Parser, ValueEnum};

mod api;
mod data;
mod validator;

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
    let mut collected_data = data::models::meta_data::CollectedData {
        collected_meta_data: Vec::new(),
    };
    for artist in &sanitized_args.artists {
        let data = setlist_fm_client
            .get_setlist_by_artist(artist, args.page_depth)
            .await;

        let setlists_from_api: Vec<data::models::setlistfm_response_models::Setlist> =
            data
                .into_iter()
                .filter_map(|res| res.ok())
                .flat_map(|resp| resp.setlist)
                .collect();

        let mut analyzed_data =
            data::setlist_data_processor::SetlistDataProcessor::reduce_to_song_stats(
                &setlists_from_api,
            );

        analyzed_data = data::setlist_data_processor::SetlistDataProcessor::calculate_mean_positions(
            &mut analyzed_data,
        );

        let data_with_meta_information = data::models::meta_data::MetaData {
            artist_name: artist.to_string(),
            song_stats: analyzed_data
        };
        
        collected_data.collected_meta_data.push(data_with_meta_information);
    }
    
    for data in collected_data.collected_meta_data {
        println!("Artist: {}", data.artist_name);
        println!("{:#?}", data.song_stats)
    };
}
