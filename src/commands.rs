use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{event, instrument, Level};

#[instrument]
pub fn encode(args: EncodeArgs) -> eyre::Result<()> {
    event!(Level::INFO, "Encoding message into PNG file");
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.into());
    let mut png: Png;
    if let Some(output_file_path) = args.output_file_path {
        fs::copy(args.file_path, &output_file_path)?;
        png = read_file(output_file_path)?;
    } else {
        png = read_file(args.file_path)?;
    }
    png.append_chunk(chunk);
    Ok(())
}

#[instrument]
pub fn decode(args: DecodeArgs) -> eyre::Result<()> {
    event!(Level::INFO, "Decoding message from PNG file");
    let png = read_file(args.file_path)?;
    if let Some(chunk) = png.chunk_by_type(args.chunk_type.as_str()) {
        let message = chunk.data_as_string()?;
        print!("{}", message);
        Ok(())
    } else {
        Err(eyre!("Chunk type {} not found", args.chunk_type))
    }
}

#[instrument]
pub fn remove(args: RemoveArgs) -> eyre::Result<()> {
    event!(Level::INFO, "Removing chunk from PNG file");
    let mut png = read_file(args.file_path)?;
    let _removed_chunk = png.remove_chunk(args.chunk_type.as_str())?;
    Ok(())
}

#[instrument]
pub fn print_chunks(args: PrintArgs) -> eyre::Result<()> {
    event!(Level::INFO, "Printing chunks from PNG file");
    let png = read_file(args.file_path)?;
    print!("{}", png);
    Ok(())
}

fn read_file(file_path: PathBuf) -> eyre::Result<Png> {
    let file = fs::read(file_path)?;
    Png::try_from(file.as_slice())
}
