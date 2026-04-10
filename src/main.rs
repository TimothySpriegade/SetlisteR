use crate::data::models::meta_data::{CollectedData, MetaData};
use crate::data::models::setlistfm_response_models::{Setlist, SetlistResponse};
use crate::data::setlist_data_processor::SetlistDataProcessor;
use crate::secrets_manager::secrets_manager::SecretsManager;
use crate::validator::arg_validator::{ArgValidator, SanitizedArgs};
use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::sync::Arc;
use tokio::task::JoinSet;

mod api;
mod data;
mod secrets_manager;
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

    /// Store the setlist.fm API key in the system keyring and exit
    #[arg(long)]
    setlist_api_key: Option<String>,
}

#[derive(ValueEnum, Clone)]
enum StreamingService {
    Spotify,
    YouTubeMusic,
}

#[tokio::main]
async fn main() {
    let sanitized_args = parse_and_validate_args();
    let api_key = resolve_api_key(&sanitized_args);
    let client = build_setlist_client(api_key);

    let raw_data =
        fetch_all_artists(client, sanitized_args.artists, sanitized_args.page_depth).await;
    let collected_data = run_analysis(raw_data);

    print_results(&collected_data);
}

fn parse_and_validate_args() -> SanitizedArgs {
    let args = Args::parse();
    match ArgValidator::validate(&args) {
        Ok(validated_args) => validated_args,
        Err(err) => {
            eprintln!("Error validating arguments: {}", err);
            std::process::exit(1);
        }
    }
}

fn resolve_api_key(sanitized_args: &SanitizedArgs) -> String {
    let mut secret_manager = SecretsManager::new();
    secret_manager
        .set_keys_from_args(sanitized_args.secret_hashmap.clone())
        .expect("Secret configuration failed");
    secret_manager
        .get_setlist_fm_api_key()
        .expect("Setlist.fm API key not found in secrets manager")
}

fn build_setlist_client(api_key: String) -> Arc<api::setlist_fm::SetlistFmClient> {
    Arc::new(api::setlist_fm::SetlistFmClient::new(api_key))
}

async fn fetch_all_artists(
    client: Arc<api::setlist_fm::SetlistFmClient>,
    artists: Vec<String>,
    page_depth: u16,
) -> Vec<(String, Vec<Result<SetlistResponse, String>>)> {
    let mut tasks = JoinSet::new();

    for artist in artists {
        let client = Arc::clone(&client);
        tasks.spawn(async move {
            let data = client.get_setlist_by_artist(&artist, page_depth).await;
            (artist, data)
        });
    }

    let mut results = Vec::new();

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok((artist, data)) => {
                results.push((artist, data));
            }
            Err(err) => {
                eprintln!("Artist task failed: {err}");
            }
        }
    }

    results
}

fn run_analysis(raw_data: Vec<(String, Vec<Result<SetlistResponse, String>>)>) -> CollectedData {
    let meta_data: Vec<MetaData> = raw_data
        .into_par_iter()
        .map(|(artist, setlist_api_data)| {
            let setlists_from_api: Vec<Setlist> = setlist_api_data
                .into_iter()
                .filter_map(|res| res.ok())
                .flat_map(|resp| resp.setlist)
                .collect();

            let mean_song_count =
                SetlistDataProcessor::average_songs_per_setlist(&setlists_from_api);
            let analyzed_data = SetlistDataProcessor::reduce_to_song_stats(&setlists_from_api);

            MetaData {
                artist_name: artist,
                mean_song_count,
                song_stats: analyzed_data,
            }
        })
        .collect();

    CollectedData {
        collected_meta_data: meta_data,
    }
}

fn print_results(collected_data: &CollectedData) {
    for data in &collected_data.collected_meta_data {
        println!("Artist: {}", data.artist_name);
        print!("Average songs per setlist: {:.2}\n", data.mean_song_count);
        println!("{:#?}", data.song_stats)
    }
}
