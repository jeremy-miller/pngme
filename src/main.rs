mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::{App, Commands};
use clap::Parser;
use color_eyre::eyre::Result;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    color_eyre::install()?;
    let _subscriber = FmtSubscriber::builder().finish();

    let app = App::parse();
    match app.command {
        Commands::Encode(args) => {
            println!("Encoding with args: {:?}", args);
        }
        Commands::Decode(args) => {
            println!("Decoding with args: {:?}", args);
        }
        Commands::Remove(args) => {
            println!("Removing with args: {:?}", args);
        }
        Commands::Print(args) => {
            println!("Printing with args: {:?}", args);
        }
    }

    Ok(())
}
