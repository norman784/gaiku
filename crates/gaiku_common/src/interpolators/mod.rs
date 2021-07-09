//! The interpolation methods
//!
//! Interpolaters are structures designed to
//! calculated values off of the usual grid
//! points. They can include smoothing and
//! are typically used to calculate LODs
//! at higher reolutions then the input data
//!
mod common;
mod nearestneighbour;

pub use self::{common::*, nearestneighbour::NearestNeighbour};
