use std::{fs, path::Path};

use clap::Parser;
use png::Png;

use crate::cli::Cli;

mod cli;
mod png;
fn main() -> Result<()> {
    let cli = Cli::parse();
    //get first file
    let input: Box<dyn ConvertibleImage> = match &cli
        .source
        .extension()
        .expect("Invalid source; files must have an extension")
        .to_str()
    {
        Some("png") => Png::from_file(&cli.source)?,
        Some(default) => {
            println!("Files with extension {} are not supported at this time; please select a different sourcefile type", default);
            return Ok(());
        }
        None => {
            println!("Unable to read file extension for source");
            return Ok(());
        }
    };

    //check second file type
    let output_bytes: Vec<u8> = match &cli
        .target
        .extension()
        .expect("Invalid target; files must have an extension")
        .to_str()
    {
        Some("png") => input.to_png(cli.flags).to_bytes(),
        Some(default) => {
            println!("Files with extension {} are not supported at this time; please select a different target file type", default);
            return Ok(());
        }
        None => {
            println!("Unable to read file extension for target");
            return Ok(());
        }
    };

    fs::write(cli.target, output_bytes)?;

    return Ok(());
}

//error handling types
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Represents an image type that can be converted to and from the PNG type
trait ConvertibleImage {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Box<Self>>
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
    fn to_png(&self, flags: Option<Vec<String>>) -> png::Png;
    fn from_png(png: png::Png) -> Self
    where
        Self: Sized;
}
