# Gaiku (WIP very early development stage)

3d agnostic framework (game engine) terrain engine. The purpose of the Gaiku is to provide a 
foundation to build your games easily without worrying about implementing your own terrain engine.

The main crate is developed game engine agnostic, so we can help to reach more developers.

## Features

General features:

-[ ] Mesh generation
  -[ ] Height map
  -[ ] Marching Cubes
  -[x] Voxel
-[ ] Mesh collider
-[ ] Texturing
-[ ] Foliage support
-[ ] Chunk based for infinite generated terrains
-[ ] Mesh optimization
-[ ] LOD support

## Examples

To run the examples go to the folder `gaiku` and run with:

```bash
cargo run --example voxel
```

Then go to the `examples/output` folder and see the exported `.obj` files.

**Voxel**

Planet

<img alt="Planet" src="images/gaiku-3d/planet.png" width="600px" />

Terrain

<img alt="Terrain" src="images/gaiku-3d/terrain.png" width="600px" />

Tree

<img alt="Tree" src="images/gaiku-3d/tree.png" width="600px" />

