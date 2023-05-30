pub enum PngChunkType {

}


pub struct Chunk {
	length: u32,
    chunk_type: PngChunkType,
    data: Vec<u8>,
    crc: u32,
}