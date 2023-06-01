use crate::compression::CompressionType;

/*
 * Most BMP files just use the BMPINFOHEADER type, so for now we'll only implement
 * that header type. The purpose of treating it as a trait is so that we can
 * implement other header types in the future if we want
 */

pub trait BmpInfoHeader {
	fn length(&self) -> u32;
	fn px_width(&self) -> i32;
	fn px_height(&self) -> i32;
	fn bits_per_pixel(&self) -> u16;
	fn compression_type<'a>(&'a self) -> Option<&'a CompressionType>;
	fn img_size(&self) -> u32;
	fn res_horiz(&self) -> i32;
	fn res_vert(&self) -> i32;
	fn num_colors(&self) -> u32;
	fn num_important_colors(&self) -> u32;
	fn as_bytes(&self) -> Vec<u8>;
}

pub struct BitmapInfoHeader {
	length: u32,
	px_width: i32,
	px_height: i32,
	bits_per_pixel: u16,
	compression_type: Option<CompressionType>,
	img_size: u32,
	res_horiz: i32,
	res_vert: i32,
	num_colors: u32,
	num_important_colors: u32
}

impl BmpInfoHeader for BitmapInfoHeader {
	fn length(&self) -> u32 {
		self.length
	}

	fn px_width(&self) -> i32 {
		self.px_width
	}

	fn px_height(&self) -> i32 {
		self.px_height
	}

	fn bits_per_pixel(&self) -> u16 {
		self.bits_per_pixel
	}

	fn compression_type<'a>(&'a self) -> Option<&'a CompressionType> {
		self.compression_type.as_ref()
	}

	fn img_size(&self) -> u32 {
		self.img_size
	}

	fn res_horiz(&self) -> i32 {
		self.res_horiz
	}

	fn res_vert(&self) -> i32 {
		self.res_vert
	}

	fn num_colors(&self) -> u32 {
		self.num_colors
	}

	fn num_important_colors(&self) -> u32 {
		self.num_important_colors
	}

	fn as_bytes(&self) -> Vec<u8> {
		let comp_bytes: u32 = match self.compression_type() {
			Some(CompressionType::BI_RLE8) => 1,
			Some(CompressionType::BI_RLE4) => 2,
			Some(_) => u32::MAX,
			None => 0
		};
		self.length.to_le_bytes()
		.iter()
		.chain(self.px_width.to_le_bytes().iter())
		.chain(self.px_height.to_le_bytes().iter())
		.chain(self.bits_per_pixel.to_le_bytes().iter())
		.chain(comp_bytes.to_le_bytes().iter())
		.chain(self.img_size.to_le_bytes().iter())
		.chain(self.res_horiz.to_le_bytes().iter())
		.chain(self.res_vert.to_le_bytes().iter())
		.chain(self.num_colors.to_le_bytes().iter())
		.chain(self.num_important_colors.to_le_bytes().iter())
		.copied()
		.collect()
	}
}