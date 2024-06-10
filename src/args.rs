use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Program to hide messages in PNG files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encodes a message into a PNG file and saves the result
    Encode(EncodeArgs),

    /// Searches for a message hidden in a PNG file and prints the message if one is found
    Decode(DecodeArgs),

    /// Removes a chunk from a PNG file
    Remove(RemoveArgs),

    /// Prints all of the chunks in a PNG file
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to input PNG file
    pub file_path: PathBuf,

    /// Chunk type of message
    pub chunk_type: String,

    /// Message to encode
    pub message: String,

    /// Optional path to output PNG file
    pub output_file_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to input PNG file
    pub file_path: PathBuf,

    /// Chunk type of message
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Path to input PNG file
    pub file_path: PathBuf,

    /// Chunk type of message
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// Path to input PNG file
    pub file_path: PathBuf,
}
