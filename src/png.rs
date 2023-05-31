pub mod chunk;
pub mod chunk_type;

use std::{fs, path::Path};

use crate::{ConvertibleImage, Error, Result};

use self::chunk::Chunk;

/// based on the design used in the PNGMe tutorial by <author>, which can be found at <link>
#[derive(Clone, Debug)]
pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Png { chunks }
    }

	 /// Lists the `Chunk`s stored in this `Png`
	 pub fn chunks(&self) -> &[Chunk] {
        &self.chunks.as_slice()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Png> {
        let mut chunks: Vec<Chunk> = Vec::new();
        if bytes.len() < 8 {
            return Err(format!(
                "not enough bytes; a valid png needs at least enough bytes for a header"
            )
            .into());
        }
        let header_bytes = &bytes[0..8];
        if !header_bytes.eq(&Png::STANDARD_HEADER) {
            return Err(format!("invalid header").into());
        }
        let mut pointer: usize = 8;
        while pointer < bytes.len() {
            //println!("Pointer is {pointer}");
            //println!("Chunk bytes are {:?}", &bytes[pointer..]);
            let chunk = Chunk::try_from(&bytes[pointer..])?;
            pointer += 12 + chunk.length() as usize;
            chunks.push(chunk);
        }
        Ok(Png { chunks })
    }
}

impl ConvertibleImage for Png {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Box<Self>> {
        let file_bytes = fs::read(path)?;
        match Png::try_from(&file_bytes[..]) {
            Ok(png) => Ok(Box::new(png)),
            Err(e) => Err(e),
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut byte_vec: Vec<u8> = Vec::new();
        byte_vec.extend_from_slice(&Png::STANDARD_HEADER);
        for chunk in &self.chunks {
            byte_vec.extend_from_slice(chunk.as_bytes().as_slice());
        }
        byte_vec
    }
    fn from_png(png: Png) -> Self {
        png
    }

    fn to_png(&self, _flags: Vec<String>) -> Png {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{*, chunk_type::PngChunkType};

	fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk> {
        let chunk_type = PngChunkType::from_str(chunk_type)?;
        let data: Vec<u8> = data.bytes().collect();

        Ok(Chunk::new(chunk_type, data))
    }

	fn testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::new();

        chunks.push(chunk_from_strings("IHDR", "I am the first chunk").unwrap());
        chunks.push(chunk_from_strings("IDAT", "I am another chunk").unwrap());
        chunks.push(chunk_from_strings("IEND", "I am the last chunk").unwrap());

        chunks
    }

	fn testing_png() -> Png {
        let chunks = testing_chunks();
        Png::from_chunks(chunks)
    }

	#[test]
    fn test_from_chunks() {
        let chunks = testing_chunks();
        let png = Png::from_chunks(chunks);

        assert_eq!(png.chunks().len(), 3);
    }

	#[test]
    fn test_valid_from_bytes() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_ok());
    }
	
	#[test]
    fn test_invalid_header() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = [13, 80, 78, 71, 13, 10, 26, 10]
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_err());
    }

	#[test]
    fn test_list_chunks() {
        let png = testing_png();
        let chunks = png.chunks();
        assert_eq!(chunks.len(), 3);
    }

	#[test]
    fn test_invalid_chunk() {
        let mut chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        #[rustfmt::skip]
        let mut bad_chunk = vec![
            0, 0, 0, 5,         // length
            32, 117, 83, 116,   // Chunk Type (bad)
            65, 64, 65, 66, 67, // Data
            1, 2, 3, 4, 5       // CRC (bad)
        ];

        chunk_bytes.append(&mut bad_chunk);

        let png = Png::try_from(chunk_bytes.as_ref());

        assert!(png.is_err());
    }
}