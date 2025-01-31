
use noise::{Perlin, NoiseFn};
use super::{Chunk, ChunkPos};

pub struct WorldGenerator {
    noise: Perlin,
    seed: u32,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            noise: Perlin::new().set_seed(seed),
            seed,
        }
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk, pos: ChunkPos) {
        let scale = 64.0;
        let amplitude = 16.0;
        let base_height = 64.0;

        for x in 0..Chunk::SIZE {
            for z in 0..Chunk::SIZE {
                let world_x = (pos.x * Chunk::SIZE as i32 + x as i32) as f64;
                let world_z = (pos.z * Chunk::SIZE as i32 + z as i32) as f64;

                let height = self.noise.get([
                    world_x / scale,
                    world_z / scale,
                ]) * amplitude + base_height;

                for y in 0..Chunk::SIZE {
                    let world_y = pos.y * Chunk::SIZE as i32 + y as i32;
                    chunk.set_block(
                        x, y, z,
                        if world_y as f64 <= height { 1 } else { 0 }
                    );
                }
            }
        }
    }
}
