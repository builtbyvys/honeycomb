use honeycomb::world::{World, Chunkpos};

fn main() {
    let world = World::new(12345);
    
    // generate initial chunks
    world.generate_chunk(ChunkPos::from_world(0, 0, 0));
    world.generate_chunk(ChunkPos::from_world(1, 0, 0));
    
    // access blocks
    let block = world.get_block(10, 64, 10);
    println!("Block at (10,64,10): {}", block);
}
