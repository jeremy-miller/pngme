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
    /// Encode a message into a PNG file
    Encode(EncodeArgs),

    /// Decode a message from a PNG file
    Decode(DecodeArgs),

    /// Remove a message from a PNG file
    Remove(RemoveArgs),

    /// Print all messages in a PNG file
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to input PNG file
    file_path: PathBuf,

    /// Chunk type of message
    chunk_type: String,

    /// Message to encode
    message: String,

    /// Optional path to output PNG file
    output_file_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to input PNG file
    file_path: PathBuf,

    /// Chunk type of message
    chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Path to input PNG file
    file_path: PathBuf,

    /// Chunk type of message
    chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// Path to input PNG file
    file_path: PathBuf,
}
