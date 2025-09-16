//! Track ID Extractor
//!
//! Helper utility to extract Spotify track IDs from various formats

use clap::{Arg, ArgMatches, Command};
use std::process;

fn main() {
    let matches = Command::new("extract-track-ids")
        .version("0.1.0")
        .about("Extract Spotify track IDs from various input formats")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INPUT_FILE")
                .help("Input file to process")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Output file for track IDs")
                .default_value("extracted_tracks.txt"),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Input format: csv, urls, or mixed")
                .default_value("mixed"),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let format = matches.get_one::<String>("format").unwrap();

    let content = std::fs::read_to_string(input_path)?;
    let mut track_ids = Vec::new();

    match format.as_str() {
        "csv" => {
            track_ids = extract_from_csv(&content)?;
        }
        "urls" => {
            track_ids = extract_from_urls(&content);
        }
        "mixed" => {
            // Try to extract from any format
            track_ids.extend(extract_from_urls(&content));
            if let Ok(csv_ids) = extract_from_csv(&content) {
                track_ids.extend(csv_ids);
            }
            // Also look for plain track IDs
            track_ids.extend(extract_plain_ids(&content));
        }
        _ => {
            return Err("Unsupported format. Use: csv, urls, or mixed".into());
        }
    }

    // Remove duplicates and empty entries
    track_ids.sort();
    track_ids.dedup();
    track_ids.retain(|id| !id.is_empty() && id.len() == 22);

    if track_ids.is_empty() {
        return Err("No valid track IDs found in input file".into());
    }

    // Write to output file
    let output_content = track_ids.join("\n");
    std::fs::write(output_path, output_content)?;

    println!(
        "Extracted {} unique track IDs from {} to {}",
        track_ids.len(),
        input_path,
        output_path
    );
    println!("First few track IDs:");
    for (i, id) in track_ids.iter().take(5).enumerate() {
        println!("  {}: {}", i + 1, id);
    }

    Ok(())
}

fn extract_from_csv(content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut track_ids = Vec::new();

    for line in content.lines() {
        // Handle CSV with potential comma-separated values
        for field in line.split(',') {
            let field = field.trim().trim_matches('"');
            if let Some(id) = extract_track_id_from_text(field) {
                track_ids.push(id);
            }
        }
    }

    Ok(track_ids)
}

fn extract_from_urls(content: &str) -> Vec<String> {
    let mut track_ids = Vec::new();

    for line in content.lines() {
        if let Some(id) = extract_track_id_from_text(line.trim()) {
            track_ids.push(id);
        }
    }

    track_ids
}

fn extract_plain_ids(content: &str) -> Vec<String> {
    let mut track_ids = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        // Check if it looks like a plain track ID (22 characters, alphanumeric)
        if line.len() == 22 && line.chars().all(|c| c.is_alphanumeric()) {
            track_ids.push(line.to_string());
        }
    }

    track_ids
}

fn extract_track_id_from_text(text: &str) -> Option<String> {
    // Extract from Spotify URLs like:
    // https://open.spotify.com/track/69Kzq3FMkDwiSFBQzRckFD
    // spotify:track:69Kzq3FMkDwiSFBQzRckFD

    if let Some(pos) = text.find("track/") {
        let start = pos + 6;
        if let Some(end_pos) = text[start..].find(|c: char| !c.is_alphanumeric()) {
            let id = &text[start..start + end_pos];
            if id.len() == 22 {
                return Some(id.to_string());
            }
        } else {
            let id = &text[start..];
            if id.len() == 22 {
                return Some(id.to_string());
            }
        }
    }

    if let Some(pos) = text.find("track:") {
        let start = pos + 6;
        if let Some(end_pos) = text[start..].find(|c: char| !c.is_alphanumeric()) {
            let id = &text[start..start + end_pos];
            if id.len() == 22 {
                return Some(id.to_string());
            }
        } else {
            let id = &text[start..];
            if id.len() == 22 {
                return Some(id.to_string());
            }
        }
    }

    // Check if the entire text is a track ID
    if text.len() == 22 && text.chars().all(|c| c.is_alphanumeric()) {
        return Some(text.to_string());
    }

    None
}
