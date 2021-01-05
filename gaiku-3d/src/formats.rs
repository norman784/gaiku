#[cfg(feature = "gox")]
mod gox;
#[cfg(feature = "png")]
mod png;

#[cfg(feature = "gox")]
pub use self::gox::GoxReader;
#[cfg(feature = "png")]
pub use self::png::PNGReader;
