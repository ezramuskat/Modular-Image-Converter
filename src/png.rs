use std::fs;

use crate::ConvertibleImage;

pub enum PngChunkType {

}

pub struct Chunk {
	length: u32,
    chunk_type: PngChunkType,
    data: Vec<u8>,
    crc: u32,
}

pub struct Png {
    chunks: Vec<Chunk>,
}

impl ConvertibleImage for Png {
	fn from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Box<Self>> {
		let file_bytes = fs::read(path);
	}
	fn from_png(png: self::Png) -> Self {
		png
	}

	fn to_png(&self) -> self::Png {
		*self
	}
}