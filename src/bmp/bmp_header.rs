use crate::Error;

#[derive(Clone, Copy, Debug)]
pub struct BmpHeader {
    length: u32,
    reserved_vals: [u8; 4],
    img_offset: u32,
}

impl BmpHeader {
    pub fn new(length: u32, reserved_vals: [u8; 4], img_offset: u32) -> BmpHeader {
        BmpHeader {
            length,
            reserved_vals,
            img_offset,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn reserved_vals(&self) -> &[u8; 4] {
        &self.reserved_vals
    }

    pub fn img_offset(&self) -> u32 {
        self.img_offset
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_le_bytes()
            .iter()
            .chain(self.reserved_vals.iter())
            .chain(&self.img_offset.to_le_bytes())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for BmpHeader {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&value[2..6]);
        let length = u32::from_le_bytes(buf);
        let mut reserved_vals: [u8; 4] = [0; 4];
        reserved_vals.copy_from_slice(&value[6..10]);
        buf.copy_from_slice(&value[10..14]);
        let img_offset = u32::from_le_bytes(buf);
        Ok(BmpHeader {
            length,
            reserved_vals,
            img_offset,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::BmpHeader;

    #[test]
    fn test_new_header() {
        let length = 25;
        let reserved_vals = [0; 4];
        let img_offset = 30;
        let header = BmpHeader::new(length, reserved_vals, img_offset);
        assert_eq!(header.length(), 25);
		assert_eq!(reserved_vals, [0;4]);
        assert_eq!(header.img_offset(), 30);
    }

    #[test]
    fn test_header_from_bytes() {
        let length: u32 = 25;
        let reserved_vals: [u8; 4] = [0; 4];
        let img_offset: u32 = 30;

        let header_data: Vec<u8> = "BM"
            .as_bytes()
            .iter()
            .chain(length.to_le_bytes().iter())
            .chain(reserved_vals.iter())
            .chain(img_offset.to_le_bytes().iter())
            .copied()
            .collect();

        let header = BmpHeader::try_from(header_data.as_ref()).unwrap();
        assert_eq!(header.length(), 25);
		assert_eq!(reserved_vals, [0;4]);
        assert_eq!(header.img_offset(), 30);
    }
}
