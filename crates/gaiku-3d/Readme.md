# Gaiku 3d

## DEPRECATED IN FAVOR OF `gaiku` CRATE

Implementation of the 3d terrain engine.

## File formats

- Goxel
- PNG

## Bakers

- Voxel
- Marching cubes
- Heightmap

## TODO

- [ ] Integrate Octree
- [ ] Implement [Four-triangles adaptive algorithms for RTIN terrain meshes](https://www.sciencedirect.com/science/article/pii/S0895717708001040)
- [ ] Implement Greedy meshing
- [ ] Make Bakers and FileFormats configurable
- [ ] Add Texture support
- [ ] Add LOD support
- [ ] Write benchmarks
- [ ] Write tests
- [ ] Integrate Rayon
- [ ] Optimize the entire process, right now it suboptimal because the loops,
need to figure it out a better way to do this.


