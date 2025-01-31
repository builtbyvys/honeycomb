mod chunk;
mod generate;

pub use chunk::Chunk;
pub use generate::WorldGenerator;

use parking_lot::RwLock;
use std::collections::HashMap;

pub struct World {
    chunks: RwLock<HashMap<ChunkPos, Chunk>>,
    generate: WorldGenerator,
    seed: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkPos {
    // convert world coords to chunk coords
    pub fn from_world(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x.div_euclid(Chunk::SIZE as i32),
            y: y.div_euclid(Chunk::SIZE as i32),
            z: z.div_euclid(Chunk::SIZE as i32),
        }
    }
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            chunks: RwLock::new(HashMap::new()),
            generate: WorldGenerator::new(seed),
            seed,
        }
    }

    // get block at world coords
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u8 {
        let chunk_pos = ChunkPos::from_world(x, y, z);
        self.chunks.read().get(&chunk_pos).map_or(0, |chunk| {
            chunk.get_block(
                x.rem_euclid(Chunk::SIZE as i32) as usize,
                y.rem_euclid(Chunk::SIZE as i32) as usize,
                z.rem_euclid(Chunk::SIZE as i32) as usize
            )
        })
    }

    // generate chunk at specified position
    pub fn generate_chunk(&self, pos: ChunkPos) {
        let mut chunk = Chunk::new();
        self.generator.generate_chunk(&mut chunk, pos);
        self.chunks.write().insert(pos, chunk);
    }

    // update world state
    pub fn update(&self, delta_time: f32) {
        // TODO: add dynamic world updates
    }

    // get world seed
    pub fn seed(&self) -> u32 {
        self.seed
    }
}
