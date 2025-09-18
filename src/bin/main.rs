//! Wave Code Generator CLI
//!
//! Simple command-line interface for generating HTML pages with Spotify wave codes from plaintext input files.

use clap::{Arg, ArgMatches, Command};
use std::process;
use wave_code_generator::{generate_wave_codes_page, load_track_ids_from_file};

fn main() {
    let matches = Command::new("wave-gen")
        .version("0.1.0")
        .author("Rust Developer")
        .about("Generate printable HTML pages with Spotify wave codes from plaintext input files")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INPUT_FILE")
                .help("Input text file with track IDs (one per line)")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Output HTML filename (saved in output/ folder)"),
        )
        .arg(
            Arg::new("title")
                .short('t')
                .long("title")
                .value_name("TITLE")
                .help("Page title")
                .default_value("Spotify Codes Printable Page"),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // Get input file path - look in input/ folder if not absolute path
    let input_file = matches.get_one::<String>("input").unwrap();
    let input_path = if input_file.starts_with('/') || input_file.contains(':') {
        input_file.clone()
    } else {
        format!("input/{}", input_file)
    };

    // Load track IDs from plaintext file
    let track_ids = load_track_ids_from_file(&input_path)?;

    if track_ids.is_empty() {
        return Err("No track IDs found in input file".into());
    }

    // Get title
    let title = matches.get_one::<String>("title").unwrap();

    // Generate HTML
    let html = generate_wave_codes_page(&track_ids, Some(title));

    // Determine output path - save in output/ folder
    let output_filename = if let Some(output) = matches.get_one::<String>("output") {
        output.clone()
    } else {
        // Generate filename based on input filename
        let input_stem = std::path::Path::new(input_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("wave_codes");
        format!("{}.html", input_stem)
    };

    let output_path = format!("output/{}", output_filename);

    // Write to output file
    std::fs::write(&output_path, html)?;

    println!(
        "Generated {} with {} tracks from {}",
        output_path,
        track_ids.len(),
        input_path
    );

    Ok(())
}
