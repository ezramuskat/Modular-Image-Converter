use std::{
    fmt::{Debug, Display},
    str::{from_utf8, FromStr},
};

/// based on the design used in the PNGMe tutorial, which can be found at <https://picklenerd.github.io/pngme_book/>
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PngChunkType {
    code: [u8; 4],
}

impl PngChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.code
    }

    pub fn is_critical(&self) -> bool {
        self.code[0] & (1 << 5) == 0
    }

    pub fn is_public(&self) -> bool {
        self.code[1] & (1 << 5) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.code[2] & (1 << 5) == 0
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        self.code[3] & (1 << 5) != 0
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl Display for PngChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(from_utf8(&self.code).unwrap(), f)
    }
}

impl TryFrom<[u8; 4]> for PngChunkType {
    type Error = String;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in value {
            if !byte.is_ascii_alphabetic() {
                return Err(format!("All bytes must be valid alphabetic ASCII; the byte {byte} in the input array {value:?} is not valid ASCII"));
            }
        }
        Ok(PngChunkType { code: value })
    }
}

impl FromStr for PngChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("Chunk types must contain 4 bytes");
        }
        for char in s.chars() {
            if !char.is_ascii_alphabetic() {
                return Err("All bytes must be valid alphabetic ASCII");
            }
        }
        let mut code: [u8; 4] = [0; 4];
        for idx in 0..4 {
            code[idx] = s.as_bytes()[idx]
        }
        Ok(PngChunkType { code })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = PngChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = PngChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = PngChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = PngChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = PngChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = PngChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = PngChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = PngChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = PngChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = PngChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = PngChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = PngChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = PngChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = PngChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = PngChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: PngChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: PngChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
