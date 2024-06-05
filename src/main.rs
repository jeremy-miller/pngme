mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use color_eyre::eyre::Result;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    color_eyre::install()?;
    let _subscriber = FmtSubscriber::builder().finish();

    todo!()
}
