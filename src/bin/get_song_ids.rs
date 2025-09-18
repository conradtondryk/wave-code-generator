//! Get Song IDs from Spotify Playlist
//!
//! Extract track IDs from a Spotify playlist URL using the Spotify Web API

use base64::Engine;
use clap::{Arg, ArgMatches, Command};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Serialize, Deserialize)]
struct SpotifyAuthResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlaylistResponse {
    tracks: PlaylistTracks,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlaylistTracks {
    items: Vec<PlaylistItem>,
    next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlaylistItem {
    track: Option<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    id: Option<String>,
    name: String,
    artists: Vec<Artist>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Artist {
    name: String,
}

#[tokio::main]
async fn main() {
    let matches = Command::new("get-song-ids")
        .version("0.1.0")
        .author("Rust Developer")
        .about("Extract track IDs from Spotify playlist URLs")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("PLAYLIST_URL")
                .help("Spotify playlist URL")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Output text file for track IDs (saved in input/ folder)")
                .default_value("playlist_tracks.txt"),
        )
        .arg(
            Arg::new("client-id")
                .long("client-id")
                .value_name("CLIENT_ID")
                .help("Spotify Client ID")
                .required(true),
        )
        .arg(
            Arg::new("client-secret")
                .long("client-secret")
                .value_name("CLIENT_SECRET")
                .help("Spotify Client Secret")
                .required(true),
        )
        .get_matches();

    if let Err(e) = run(&matches).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let playlist_url = matches.get_one::<String>("url").unwrap();
    let client_id = matches.get_one::<String>("client-id").unwrap();
    let client_secret = matches.get_one::<String>("client-secret").unwrap();
    let output_filename = matches.get_one::<String>("output").unwrap();

    // Extract playlist ID from URL
    let playlist_id = extract_playlist_id(playlist_url)?;
    println!("Extracted playlist ID: {}", playlist_id);

    // Get Spotify access token
    let client = Client::new();
    let access_token = get_access_token(&client, client_id, client_secret).await?;
    println!("Successfully authenticated with Spotify API");

    // Get playlist tracks
    let track_ids = get_playlist_tracks(&client, &access_token, &playlist_id).await?;

    if track_ids.is_empty() {
        return Err("No tracks found in playlist".into());
    }

    println!("Found {} tracks in playlist", track_ids.len());

    // Save to input folder
    let output_path = format!("input/{}", output_filename);
    let content = track_ids.join("\n");
    std::fs::write(&output_path, content)?;

    println!("Saved track IDs to {}", output_path);
    println!("\nFirst few track IDs:");
    for (i, id) in track_ids.iter().take(5).enumerate() {
        println!("  {}: {}", i + 1, id);
    }

    println!("\nNow you can generate HTML with:");
    println!(
        "cargo run --bin wave-gen -- -i {} -t \"My Playlist\"",
        output_filename
    );

    Ok(())
}

fn extract_playlist_id(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Handle different Spotify URL formats:
    // https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M
    // https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M?si=...

    if let Some(start) = url.find("/playlist/") {
        let id_start = start + 10; // "/playlist/".len()
        let remaining = &url[id_start..];

        // Find end of ID (before ? or end of string)
        let id_end = remaining.find('?').unwrap_or(remaining.len());
        let playlist_id = &remaining[..id_end];

        if playlist_id.len() == 22 {
            Ok(playlist_id.to_string())
        } else {
            Err("Invalid playlist ID length".into())
        }
    } else {
        Err(
            "Could not extract playlist ID from URL. Please provide a valid Spotify playlist URL."
                .into(),
        )
    }
}

async fn get_access_token(
    client: &Client,
    client_id: &str,
    client_secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let auth_header = base64::engine::general_purpose::STANDARD
        .encode(format!("{}:{}", client_id, client_secret));

    let params = [("grant_type", "client_credentials")];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", auth_header))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("Failed to get access token: {}", error_text).into());
    }

    let auth_response: SpotifyAuthResponse = response.json().await?;
    Ok(auth_response.access_token)
}

async fn get_playlist_tracks(
    client: &Client,
    access_token: &str,
    playlist_id: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut track_ids = Vec::new();
    let mut next_url = Some(format!(
        "https://api.spotify.com/v1/playlists/{}/tracks?fields=items(track(id,name,artists(name))),next&limit=100",
        playlist_id
    ));

    while let Some(url) = next_url {
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to get playlist tracks: {}", error_text).into());
        }

        let playlist_response: PlaylistTracks = response.json().await?;

        for item in playlist_response.items {
            if let Some(track) = item.track {
                if let Some(id) = track.id {
                    track_ids.push(id);
                    let artist_names: Vec<String> =
                        track.artists.iter().map(|a| a.name.clone()).collect();
                    println!("  Found: {} - {}", artist_names.join(", "), track.name);
                }
            }
        }

        next_url = playlist_response.next;
    }

    Ok(track_ids)
}
