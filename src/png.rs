pub mod chunk;

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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut byte_vec: Vec<u8> = Vec::new();
        byte_vec.extend_from_slice(&Png::STANDARD_HEADER);
        for chunk in &self.chunks {
            byte_vec.extend_from_slice(chunk.as_bytes().as_slice());
        }
        byte_vec
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
    fn from_file<P: AsRef<Path>>(path: P, flags: Vec<String>) -> Result<Box<Self>> {
        let file_bytes = fs::read(path)?;
        match Png::try_from(&file_bytes[..]) {
            Ok(png) => Ok(Box::new(png)),
            Err(e) => Err(e),
        }
    }
    fn to_file<P: AsRef<Path>>(&self, path: P, flags: Vec<String>) -> std::io::Result<()> {
        fs::write(path, &self.as_bytes())
    }
    fn from_png(png: Png) -> Self {
        png
    }

    fn to_png(&self) -> Png {
        self.clone()
    }
}
