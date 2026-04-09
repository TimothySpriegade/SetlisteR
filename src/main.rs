use crate::data::models::meta_data::{CollectedData, MetaData};
use crate::data::models::setlistfm_response_models::{Setlist, SetlistResponse};
use crate::data::setlist_data_processor::SetlistDataProcessor;
use crate::validator::arg_validator::ArgValidator;
use clap::{Parser, ValueEnum};
use std::sync::Arc;
use tokio::task::JoinSet;

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

    let api_key = load_api_key();

    let sanitized_args = match ArgValidator::validate(&args) {
        Ok(validated_args) => validated_args,
        Err(err) => {
            eprintln!("Error validating arguments: {}", err);
            std::process::exit(1);
        }
    };

    let setlist_fm_client = Arc::new(api::setlist_fm::SetlistFmClient::new(api_key));

    let collected_data =
        fetch_all_artists(setlist_fm_client, sanitized_args.artists, args.page_depth).await;

    print_results(&collected_data);
}

fn load_api_key() -> String {
    dotenvy::dotenv().ok();
    std::env::var("SETLIST_FM_API_KEY").expect("SETLIST_FM_API_KEY must be set")
}

async fn fetch_all_artists(
    client: Arc<api::setlist_fm::SetlistFmClient>,
    artists: Vec<String>,
    page_depth: u16,
) -> CollectedData {
    let mut tasks = JoinSet::new();

    for artist in artists {
        let client = Arc::clone(&client);
        tasks.spawn(async move {
            let data = client.get_setlist_by_artist(&artist, page_depth).await;
            (artist, data)
        });
    }

    let mut collected_data = CollectedData {
        collected_meta_data: Vec::new(),
    };

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok((artist, data)) => {
                run_analysis(&mut collected_data, data, &artist);
            }
            Err(err) => {
                eprintln!("Artist task failed: {err}");
            }
        }
    }

    collected_data
}

fn print_results(collected_data: &CollectedData) {
    for data in &collected_data.collected_meta_data {
        println!("Artist: {}", data.artist_name);
        print!("Average songs per setlist: {:.2}\n", data.mean_song_count);
        println!("{:#?}", data.song_stats)
    }
}

fn run_analysis(
    collection_list: &mut CollectedData,
    setlist_api_data: Vec<Result<SetlistResponse, String>>,
    artist: &String,
) {
    let setlists_from_api: Vec<Setlist> = setlist_api_data
        .into_iter()
        .filter_map(|res| res.ok())
        .flat_map(|resp| resp.setlist)
        .collect();

    let mean_song_count = SetlistDataProcessor::average_songs_per_setlist(&setlists_from_api);

    let analyzed_data = SetlistDataProcessor::reduce_to_song_stats(&setlists_from_api);

    let data_with_meta_information = MetaData {
        artist_name: artist.to_string(),
        mean_song_count,
        song_stats: analyzed_data,
    };

    collection_list
        .collected_meta_data
        .push(data_with_meta_information);
}
