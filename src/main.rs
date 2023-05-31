use std::{io, path::Path};

use clap::Parser;

use crate::cli::Cli;

mod png;
mod cli;
fn main() {
    let cli = Cli::parse();
    //get first file
    let input: Box<dyn ConvertibleImage> = match &cli.source.extension().expect("Invalid source; files must have an extension").to_str() {

        Some(default) => {
            println!("Files with extension {} are not supported at this time", default);
            return;
        },
        None => {
            println!("Unable to read file extension");
            return;
        }
    };
}

//error handling types
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Represents an image type that can be converted to and from the PNG type
trait ConvertibleImage {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Box<Self>> where Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
    fn to_png(&self, flags: Vec<String>) -> png::Png;
    fn from_png(png: png::Png) -> Self where Self: Sized;
}
