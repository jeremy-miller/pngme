mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::{App, Commands};
use crate::commands::{decode, encode, print_chunks, remove};
use clap::Parser;
use color_eyre::eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

fn main() -> Result<()> {
    dotenvy::dotenv()?;
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(ErrorLayer::default())
        .init();

    let app = App::parse();
    match app.command {
        Commands::Encode(args) => {
            encode(args)?;
        }
        Commands::Decode(args) => {
            decode(args)?;
        }
        Commands::Remove(args) => {
            remove(args)?;
        }
        Commands::Print(args) => {
            print_chunks(args)?;
        }
    }

    Ok(())
}
