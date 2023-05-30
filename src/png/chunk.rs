use std::{
    fmt,
    str::{from_utf8, FromStr},
};

use crc::CRC_32_ISO_HDLC;
use strum::{Display, EnumString};

#[derive(Display, EnumString)]
pub enum PngChunkType {
    IHDR,
    InvalidType, //not achievable through standard, used for error checking
}

pub struct Chunk {
    length: u32,
    chunk_type: PngChunkType,
    data: Vec<u8>,
    crc: u32,
}

fn generate_crc(data: &[u8]) -> u32 {
    let crc_gen = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC);
    crc_gen.checksum(data)
}

impl Chunk {
    pub fn new(chunk_type: PngChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len().try_into().unwrap();
        let full_data_bytes: Vec<u8> = chunk_type
            .to_string()
            .as_bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc = generate_crc(&full_data_bytes[..]);
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &PngChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.to_string().as_bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}
impl TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let mut buf: [u8; 4] = [0; 4];
        //println!("length of value is {}", value.len());
        buf.copy_from_slice(&value[0..4]);
        let length = u32::from_be_bytes(buf);
        buf.copy_from_slice(&value[4..8]);

        let chunk_type = match PngChunkType::from_str(
            from_utf8(&buf).unwrap_or_else(|error| return "InvalidType"),
        ) {
            Ok(PngChunkType::InvalidType) => return Err(format!("Invalid chunk type")),
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        };
        let crc_offset: usize = 8 + length as usize;
        buf.copy_from_slice(&value[crc_offset..crc_offset + 4]);
        let crc = u32::from_be_bytes(buf);
        let data = value[8..crc_offset].to_vec();

        let actual_crc = generate_crc(&value[4..crc_offset]);
        if actual_crc != crc {
            return Err(format!(
                "Invalid crc; the passed crc was {crc} but the actual crc should be {actual_crc}"
            ));
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(from_utf8(&self.data).unwrap(), f)
    }
}
