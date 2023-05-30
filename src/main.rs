use std::{io, path::Path};

mod png;
fn main() {
    println!("Hello, world!");
}

trait ConvertibleImage {
    fn from_file<P: AsRef<Path>>(path: P, flags: Vec<String>) -> io::Result<Box<Self>>;
    fn to_file<P: AsRef<Path>>(&self, path: P, flags: Vec<String>);
    fn to_png(&self) -> png::Png;
    fn from_png(png: png::Png) -> Self;
}
