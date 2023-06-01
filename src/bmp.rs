
use crate::{ConvertibleImage, Error};

use self::{bmp_file_header::BmpFileHeader, bmp_info_header::{BmpInfoHeader, BitmapInfoHeader}};

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

		let mut buf: [u8;4] = [0;4];
		buf.copy_from_slice(&value[14..18]);
		let info_header: Box<dyn BmpInfoHeader> = match u32::from_le_bytes(buf){
			40 => {
				let boxed = BitmapInfoHeader::try_from(&value[14..54])?;
				Box::new(boxed)
			},
			_ => return Err(format!("unknown header type").into())
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
		
		Ok(Bmp{ file_header, info_header, color_table, data})


	}
}