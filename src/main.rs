mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process::exit,
    str::FromStr,
};

use crate::commands::Args;
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use commands::Commands;
use png::Png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn png_from_file(file: &str) -> Result<Png> {
    let fpath = PathBuf::from(file);
    if !fpath.exists() || fpath.is_dir() {
        eprintln!("Please provide valid the path of png file");
        exit(1)
    }
    let mut f = File::options().read(true).open(&fpath)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    if buffer[..Png::STANDARD_HEADER.len()] != Png::STANDARD_HEADER {
        eprintln!("Not a valid PNG file");
        exit(1)
    }
    Ok(Png::try_from(buffer.as_slice())?)
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Some(val) => match val {
            Commands::Encode {
                file,
                chunktype,
                message,
                output_path,
            } => {
                let mut png = png_from_file(&file)?;
                let chunk = Chunk::new(
                    ChunkType::from_str(&chunktype)?,
                    message.as_bytes().to_vec(),
                );
                png.append_chunk(chunk);
                let out_path;
                if let Some(path) = output_path {
                    out_path = path
                } else {
                    out_path = file
                }
                let mut f = File::create(out_path)?;
                f.write_all(&png.as_bytes())?;
            }
            Commands::Decode { file, chunktype } => {
                let png = png_from_file(&file)?;
                if let Some(val) = png.chunk_by_type(&chunktype) {
                    println!("{}", val.data_as_string().unwrap());
                } else {
                    eprintln!("{} wasnt found in the png", chunktype)
                }
            }
            Commands::Remove { file, chunktype } => {
                let mut png = png_from_file(&file)?;
                match png.remove_first_chunk(&chunktype) {
                    Some(_) => println!("{chunktype} is removed"),
                    None => {
                        eprintln!("{} wasnt found in the png", chunktype)
                    }
                }
                let mut f = File::create(file)?;
                f.write_all(&png.as_bytes())?;
            }
            Commands::Print { file } => {
                let png = png_from_file(&file)?;
                println!("{}", png);
            }
        },
        None => todo!(),
    }
    Ok(())
}
