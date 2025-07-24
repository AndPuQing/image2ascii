use anyhow::Result;
use clap::Parser;
use colored::*;
use image2ascii::{image_to_ascii_art, AsciiArtOutput};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// A simple CLI to convert images to ASCII art
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the input image file
    #[arg(short, long)]
    input: PathBuf,

    /// Downsample rate for the image. Higher values mean smaller output.
    #[arg(short, long, default_value_t = 8)]
    downsample_rate: u32,

    /// Sobel edge detection threshold.
    #[arg(short, long, default_value_t = 50)]
    edge_sobel_threshold: u32,

    /// ASCII characters to use for edges, from dark to light.
    #[arg(long, default_value_t = String::from(" -/|\\"))]
    ascii_chars_edge: String,

    /// ASCII characters for grayscale mapping, from dark to light.
    #[arg(long, default_value_t = String::from("@?OPoc:. "))]
    ascii_chars_gray: String,
}

fn main() -> Result<()> {
    colored::control::set_override(true);
    let cli = Cli::parse();

    // --- 1. Load Image ---
    let image_bytes = fs::read(&cli.input)?;
    let img = image::load_from_memory(&image_bytes)?;

    // --- 2. Prepare ASCII character sets ---
    let ascii_chars_edge: Vec<char> = cli.ascii_chars_edge.chars().collect();
    let ascii_chars_gray: Vec<char> = cli.ascii_chars_gray.chars().collect();

    // --- 3. Call Core Logic ---
    let ascii_art_result: AsciiArtOutput = image_to_ascii_art(
        &img,
        cli.downsample_rate,
        cli.edge_sobel_threshold,
        &ascii_chars_edge,
        &ascii_chars_gray,
    ).map_err(|e| anyhow::anyhow!(e))?;

    // --- 4. Print to Console ---
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for line in ascii_art_result.lines {
        for char_info in line {
            let s = char_info
                .char
                .to_string()
                .truecolor(char_info.r, char_info.g, char_info.b);
            write!(handle, "{}", s)?;
        }
        writeln!(handle)?;
    }

    Ok(())
}