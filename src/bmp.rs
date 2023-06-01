use std::{fs, str::from_utf8};

use crate::{compression::CompressionType, ConvertibleImage, Error};

use self::{
    bmp_file_header::BmpFileHeader,
    bmp_info_header::{BitmapInfoHeader, BmpInfoHeader},
};

pub mod bmp_file_header;
pub mod bmp_info_header;

pub struct Bmp {
    file_header: BmpFileHeader,
    info_header: Box<dyn BmpInfoHeader>,
    color_table: Option<Vec<u8>>,
    data: Vec<u8>,
}

impl Bmp {
    pub fn file_header(&self) -> &BmpFileHeader {
        &self.file_header
    }

    pub fn info_header(&self) -> &Box<dyn BmpInfoHeader> {
        &self.info_header
    }

    pub fn color_table(&self) -> Option<&Vec<u8>> {
        self.color_table.as_ref()
    }

    pub fn image_data(&self) -> &[u8] {
        &self.data
    }
}

impl TryFrom<&[u8]> for Bmp {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let file_header = BmpFileHeader::try_from(&value[0..14])?;

        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&value[14..18]);
        let info_header: Box<dyn BmpInfoHeader> = match u32::from_le_bytes(buf) {
            40 => {
                let boxed = BitmapInfoHeader::try_from(&value[14..54])?;
                Box::new(boxed)
            }
            _ => return Err(format!("unknown header type").into()),
        };

        let color_table: Option<Vec<u8>>;

        if info_header.bits_per_pixel() == 24 {
            color_table = None;
        } else {
            let color_table_start = 14 + info_header.length() as usize;
            let color_table_end = color_table_start + info_header.num_colors() as usize;
            color_table = Some(value[color_table_start..color_table_end].to_vec());
        };

        let data = value[file_header.img_offset() as usize..].to_vec();

        Ok(Bmp {
            file_header,
            info_header,
            color_table,
            data,
        })
    }
}

impl ConvertibleImage for Bmp {
    fn from_file<P: AsRef<std::path::Path>>(path: P) -> crate::Result<Box<Self>>
    where
        Self: Sized,
    {
        let file_bytes = fs::read(path)?;
        match Bmp::try_from(&file_bytes[..]) {
            Ok(bmp) => Ok(Box::new(bmp)),
            Err(e) => Err(e),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.file_header
            .as_bytes()
            .iter()
            .chain(self.info_header.as_bytes().iter())
            .chain(self.color_table.iter().flatten())
            .chain(self.data.iter())
            .copied()
            .collect()
    }

    fn from_png(png: crate::png::Png) -> Self
    where
        Self: Sized,
    {
        //generate infoheader
        let png_header_bytes = png.header_chunk().unwrap().data();
        let buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&png_header_bytes[0..4]);
        let px_width: i32 = u32::from_be_bytes(buf) as i32;
        buf.copy_from_slice(&png_header_bytes[4..8]);
        let px_height: i32 = u32::from_be_bytes(buf) as i32;

        let color_type = png_header_bytes[10];

        /*
         * Some of these values should never occur under normal transfer circumstances
         * 4 in particular doesn't match up neatly with how most BMP interpreters
         * will actually work; as such, it will only ever occur when directly
         * converting a PNG file to a BMP file
         */
        let bits_per_pixel: u16 = match color_type {
            0 => png_header_bytes[9] as u16,
            2 => png_header_bytes[9] as u16 * 3,
            3 => 8,
            4 => png_header_bytes[9] as u16 * 2,
            6 => 32,
        };

        let num_colors: u32 = (2 as u32).pow(bits_per_pixel as u32);

        let info_header = BitmapInfoHeader::new(
            px_width,
            px_height,
            bits_per_pixel,
            None,
            (px_width * px_height) as u32,
            8,
            8,
            num_colors,
            0,
        );
    }
}
