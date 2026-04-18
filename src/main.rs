use crate::data::models::args::Args;
use crate::data::models::meta_data::{CollectedData, MetaData};
use crate::data::models::playlist_data::PlaylistData;
use crate::data::models::setlistfm_response::{Setlist, SetlistResponse};
use crate::data::setlist_data_processor::SetlistDataProcessor;
use crate::data::models::args::StreamingService;
use crate::secrets_manager::secrets_manager::SecretsManager;
use crate::validator::arg_validator::{ArgValidator, SanitizedArgs};
use clap::Parser;
use rayon::prelude::*;
use std::sync::Arc;
use tokio::task::JoinSet;
use crate::data::setlist_data_reducer::SetlistDataReducer;

mod api;
mod data;
mod secrets_manager;
mod validator;

#[tokio::main]
async fn main() {
    let sanitized_args = parse_and_validate_args();
    let api_key = resolve_api_key(&sanitized_args);
    let client = build_setlist_client(api_key);

    let raw_data =
        fetch_all_artists(client, sanitized_args.artists, sanitized_args.page_depth).await;
    let collected_data = run_analysis(raw_data);

    let reduced_to_setlist_data =
        SetlistDataReducer::new(sanitized_args.playlist_name, sanitized_args.service, collected_data)
            .reduce();

    print_results(&reduced_to_setlist_data);
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
    let secret_manager = SecretsManager::new();
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

fn print_results(playlist_data: &PlaylistData) {
    println!("Playlist: {}", playlist_data.playlist_name);

    if !playlist_data.platforms.is_empty() {
        let platforms = playlist_data
            .platforms
            .iter()
            .map(streaming_service_name)
            .collect::<Vec<&str>>()
            .join(", ");
        println!("Platforms: {}", platforms);
    }

    println!();

    for artist_data in &playlist_data.artist_song_data {
        println!("Artist: {}", artist_data.artist);

        for (position, song_name) in &artist_data.songs {
            println!("{}. {}", position, song_name);
        }

        println!();
    }
}

fn streaming_service_name(service: &StreamingService) -> &'static str {
    match service {
        StreamingService::Spotify => "Spotify",
        StreamingService::YouTubeMusic => "YouTube Music",
    }
}
