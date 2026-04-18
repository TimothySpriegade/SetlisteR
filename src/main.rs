use crate::data::models::args::Args;
use crate::data::models::meta_data::{ArtistAnalysis, ArtistAnalysisCollection};
use crate::data::models::playlist_data::PlaylistData;
use crate::data::models::setlistfm_response::{Setlist, SetlistResponse};
use crate::data::models::args::StreamingService;
use crate::data::setlist_data_processor::SetlistDataProcessor;
use crate::data::setlist_data_reducer::SetlistDataReducer;
use crate::secrets_manager::secrets_manager::SecretsManager;
use crate::validator::arg_validator::{ArgValidator, SanitizedArgs};
use clap::Parser;
use rayon::prelude::*;
use std::sync::Arc;
use tokio::task::JoinSet;

mod api;
mod data;
mod secrets_manager;
mod validator;

#[tokio::main]
async fn main() {
    let sanitized_args = parse_and_validate_args();
    let api_key = resolve_api_key(&sanitized_args);
    let client = build_setlist_client(api_key);

    let artist_setlist_responses =
        fetch_setlists_for_artists(client, sanitized_args.artists, sanitized_args.page_depth).await;
    let artist_analysis_collection = analyze_artist_setlists(artist_setlist_responses);

    let reduced_playlist_data = SetlistDataReducer::new(
        sanitized_args.playlist_name,
        sanitized_args.service,
        artist_analysis_collection,
    )
    .reduce();

    print_results(&reduced_playlist_data);
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
        .set_keys_from_args(sanitized_args.secrets_by_type.clone())
        .expect("Secret configuration failed");
    secret_manager
        .get_setlist_fm_api_key()
        .expect("Setlist.fm API key not found in secrets manager")
}

fn build_setlist_client(api_key: String) -> Arc<api::setlist_fm::SetlistFmClient> {
    Arc::new(api::setlist_fm::SetlistFmClient::new(api_key))
}

async fn fetch_setlists_for_artists(
    client: Arc<api::setlist_fm::SetlistFmClient>,
    artists: Vec<String>,
    page_depth: u16,
) -> Vec<(String, Vec<Result<SetlistResponse, String>>)> {
    let mut tasks = JoinSet::new();

    for artist in artists {
        let client = Arc::clone(&client);
        tasks.spawn(async move {
            let setlist_responses = client.fetch_setlists_by_artist(&artist, page_depth).await;
            (artist, setlist_responses)
        });
    }

    let mut artist_setlist_responses = Vec::new();

    while let Some(join_result) = tasks.join_next().await {
        match join_result {
            Ok((artist, setlist_responses)) => {
                artist_setlist_responses.push((artist, setlist_responses));
            }
            Err(err) => {
                eprintln!("Artist task failed: {err}");
            }
        }
    }

    artist_setlist_responses
}

fn analyze_artist_setlists(
    artist_setlist_responses: Vec<(String, Vec<Result<SetlistResponse, String>>)>,
) -> ArtistAnalysisCollection {
    let artist_analyses: Vec<ArtistAnalysis> = artist_setlist_responses
        .into_par_iter()
        .map(|(artist, setlist_responses)| {
            let setlists: Vec<Setlist> = setlist_responses
                .into_iter()
                .filter_map(|response| response.ok())
                .flat_map(|response| response.setlist)
                .collect();

            let average_songs_per_setlist =
                SetlistDataProcessor::average_songs_per_setlist(&setlists);
            let song_stats_by_name = SetlistDataProcessor::reduce_to_song_stats(&setlists);

            ArtistAnalysis {
                artist_name: artist,
                average_songs_per_setlist,
                song_stats_by_name,
            }
        })
        .collect();

    ArtistAnalysisCollection {
        artist_analyses,
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

    for artist_playlist in &playlist_data.artist_playlists {
        println!("Artist: {}", artist_playlist.artist);

        for (position, song_name) in &artist_playlist.songs_by_position {
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
