mod meshbuilder;
mod notree;
mod octree;
mod rstar;

pub use self::{
  meshbuilder::MeshBuilder, notree::NoTreeBuilder, octree::OctMeshBuilder, rstar::RstarMeshBuilder,
};

pub type DefaultMeshBuilder = OctMeshBuilder;
