use gaiku_common::{
    Chunk,
    Fileformat,
};

use gox::{
    Block,
    Data,
    Gox,
    Only,
};

use std::fs::File;

pub struct GoxReader;

impl Fileformat for GoxReader {
    fn load(stream: &mut File) -> Vec<Chunk> {
        let gox = Gox::new(stream, vec![Only::Layers, Only::Blocks]);
        let mut result = vec![];
        let mut block_data: Vec<&Block> = vec![];

        for data in gox.data.iter() {
            match &data {
                Data::Blocks(data) => {
                    block_data.push(data);
                }
                _ => {}
            }
        }

        for data in gox.data.iter() {
            match &data {
                Data::Layers(layers, _bounds) => {
                    for layer in layers.iter() {
                        if layer.blocks.len() > 0 {
                            for data in layer.blocks.iter() {
                                let block = block_data[data.block_index];
                                let mut chunk = Chunk::new(
                                    [data.x, data.y, data.z],
                                    16,
                                    16,
                                    16
                                );

                                for x in 0..chunk.width() {
                                    for y in 0..chunk.height() {
                                        for z in 0..chunk.depth() {
                                            if !block.is_empty(x, y, z) {
                                                chunk.add(x, y, z, 1.0)
                                            }
                                        }
                                    }
                                }

                                result.push(chunk);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        result
    }
}