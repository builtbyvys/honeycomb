#[derive(Debug, Clone)]
pub struct Chunk {
    blocks: Vec<u8>,
}

impl Chunk {
    pub const SIZE: usize = 32;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;

    /// new empty chunk
    pub fn new() -> Self {
        Self {
            blocks: vec![0; Self::VOLUME],
        }
    }

    /// get block at local chunk coords
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u8 {
        self.blocks[Self::block_index(x, y, z)]
    }

    /// set block at local chunk coords
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: u8) {
        let idx = Self::block_index(x, y, z);
        self.blocks[idx] = block;
    }

    /// convert 3D coordinates to 1D index
    fn block_index(x: usize, y: usize, z: usize) -> usize {
        z * Self::SIZE * Self::SIZE + y * Self::SIZE + x
    }
}
