use self::{bmp_file_header::BmpFileHeader, bmp_info_header::BmpInfoHeader};

pub mod bmp_file_header;
pub mod bmp_info_header;

pub struct Bmp {
	file_header: BmpFileHeader,
	info_header: Box<dyn BmpInfoHeader>,
	color_table: Option<[u8;3]>,
	data: Vec<u8>,
}

impl Bmp {
	pub fn file_header(&self) -> &BmpFileHeader {
		&self.file_header
	}

	pub fn info_header(&self) -> &Box<dyn BmpInfoHeader> {
		&self.info_header
	}

	pub fn color_table(&self) -> Option<&[u8;3]> {
		self.color_table.as_ref()
	}

	pub fn image_data(&self) -> &[u8] {
		&self.data
	}
}