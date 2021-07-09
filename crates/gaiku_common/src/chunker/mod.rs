//! The `chunker` module contains structures
//! that will seperate data input into
//! multiple chunks.
//!
mod common;
mod flat;

pub use self::{common::*, flat::FlatChunker};
