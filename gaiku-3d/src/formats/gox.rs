use gaiku_common::{
    Chunk,
    Fileformat,
};

use gox::{
    ChunkData,
    Gox,
    Only,
};

use std::fs::File;

pub struct GoxReader;

impl Fileformat for GoxReader {
    fn load(stream: &File) -> Vec<Chunk> {
        let gox = Gox::new(stream, vec![Only::Layer]);
        let mut result = vec![];
        let chunk_size = 16;

//        println!("{:#?}", gox);

        for data in gox.data {
            match data.data {
                ChunkData::Layer(layer) => {
                    if layer.blocks.len() > 0 {
                        let mut chunk = Chunk::new(chunk_size, false);

                        for block in layer.blocks {
                            let x = (block.x + chunk_size as i32) as usize;
                            let y = (block.y + chunk_size as i32) as usize;
                            let z = (block.z + chunk_size as i32) as usize;

                            chunk.add((x, y, z), 1.0);
                        }

                        result.push(chunk);
                    }
                }
                _ => {}
            }
        }

        result
    }
}