# Gaiku (WIP very early development stage)

[2d](gaiku-2d) and [3d](gaiku-3d) agnostic framework (game engine) terrain engine. The purpose of the Gaiku is to provide a 
foundation to build your games easily without worrying about implementing your own terrain engine.

The main crate is developed game engine agnostic, so we can help to reach more developers.

## Supported game engines

- [Amethyst](gaiku-amethyst)

## Features

General features:

- Mesh generation
  - Voxel
  - Height map
- Mesh collider
- Texturing
- Foliage support
- Chunk based for infinite generated terrains
- Mesh optimization
- LOD support
- For procedural terrain generation check the 2d and 3d `Bakers` documentations.

For specific features check [2d](gaiku-2d/Readme.md) and [3d](gaiku-3d/Readme.md)

## Example

## License

Licensed under either of

Apache License, Version 2.0, ([license/APACHE](license/APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
MIT license ([license/MIT](license/MIT) or http://opensource.org/licenses/MIT)

at your option.