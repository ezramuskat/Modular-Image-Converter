use std::{io, path::Path};

mod png;
mod cli;
fn main() {
    println!("Hello, world!");
}

//error handling types
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Represents an image type that can be converted to and from the PNG type
trait ConvertibleImage {
    fn from_file<P: AsRef<Path>>(path: P, flags: Vec<String>) -> Result<Box<Self>>;
    fn to_file<P: AsRef<Path>>(&self, path: P, flags: Vec<String>) -> io::Result<()>;
    fn to_png(&self) -> png::Png;
    fn from_png(png: png::Png) -> Self;
}
