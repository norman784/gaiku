use crate::boundary::Boundary;
use thiserror::Error;

/// Errors that the interpolator can raise
#[derive(Error, Debug)]
pub enum InterpolaterError {
  /// Interpolators can only approximate values inside its bounds
  #[error("the requested point {point:?} is out of bounds {boundary:?}")]
  OutOfBounds { point: [f32; 3], bounds: Boundary },
  /// Used for any other error a trait might need to raise
  #[error("the following error occured: {0}")]
  Other(String),
}

/// Interpolater trait describes the generic
/// interface for the interpolating methods
pub trait Interpolater<T, U> {
  /// Creates an interpolator from a chunk
  ///
  /// # Arguments
  ///
  /// * `chunk` - The chunk to load the sample data from. Data is copied
  ///
  fn from_chunk<D>(chunk: &D) -> Self
  where
    D: Chunkify<T> + Atlasify<U> + Sizable,
    Self: Sized,
  {
    let width = chunk.width();
    let height = chunk.height();
    let depth = chunk.depth();
    let mut data = vec![];
    let mut atlas_data = vec![];
    for x in 0..width {
      for y in 0..height {
        for z in 0..depth {
          data.push(chunk.get(x.into(), y.into(), z.into()));
          atlas_data.push(chunk.get_atlas(x.into(), y.into(), z.into()));
        }
      }
    }

    Self::from_array_with_atlas(
      data.as_slice(),
      atlas_data.as_slice(),
      width.try_into().unwrap(),
      height.try_into().unwrap(),
      depth.try_into().unwrap(),
    )
  }

  ///
  /// Create an interpolator from an array of Data
  ///
  /// The length of the array should match the width*height*depth
  ///
  /// # Parameters
  ///
  /// * `data` - The input data samples. These should be on a flattened grid of width x height x depth
  ///
  /// * `width` - width of data samples
  ///
  /// * `height` - height of data samples
  ///
  /// * `depth` - depth of data samples
  ///
  /// # Returns
  ///
  /// returns an Interpolator
  ///
  fn from_array(data: &[T], width: usize, height: usize, depth: usize) -> Self
  where
    Self: Sized,
  {
    Self::from_array_with_atlas(data, &[], width, height, depth)
  }

  ///
  /// Creates an interpolator with both interpolated values and atlases
  ///
  /// The length of values should equate to width*height*depth
  /// The length of atlas_data should also be the same as the values
  /// but in the event it is not the atlas will return None at that point
  ///
  /// # Parameters
  ///
  /// * `data` - The input value samples arranged on a flattened grid of width x height x depth
  ///
  /// * `atlas_data` - The input atlas values samples also arranged on a flattened grid.
  ///
  /// * `width` - width of sample data
  ///
  /// * `height` - height of sample data
  ///
  /// * `depth` - depth of sample data
  ///
  /// # Returns
  ///
  /// returns an Interpolater
  ///
  fn from_array_with_atlas(
    data: &[T],
    atlas_data: &[U],
    width: usize,
    height: usize,
    depth: usize,
  ) -> Self
  where
    Self: Sized;

  ///
  /// The boundary for which the interpolation is valid
  ///
  /// Interpolation is not the same as extrapolation
  /// The data must be contained within the samples
  /// OutOfBounds
  ///
  /// # Returns
  ///
  /// A `Boundary` which extends over the samples domain + an `EPSILON`
  ///
  fn get_boundary(&self) -> Boundary;

  ///
  /// Get the raw data sample
  ///
  /// This returns the value at a grid point uninterpolated.
  ///
  /// # Parameters
  ///
  /// * `point` - The position as a `[usize; 3]`
  ///
  /// # Returns
  ///
  /// A `Result` that is either
  ///   - OK sample value at grid point
  ///   - Err An `InterpolaterError`
  ///
  fn get_sample(&self, point: [usize; 3]) -> Result<T, InterpolaterError>;

  ///
  /// Get the interpolated value
  ///
  /// This gets the interpolated value the `point` must be in bounds.
  ///
  /// # Parameters
  ///
  /// * `point` - The point to get the interpolated value as `[f32; 3]`
  ///
  /// # Returns
  ///
  /// A `Result` that is either
  ///   - OK interpolated value at point
  ///   - Err An `InterpolaterError`
  ///
  fn get_value(&self, point: [f32; 3]) -> Result<T, InterpolaterError>;

  ///
  /// Get the raw atlas sample
  ///
  /// This returns the atlas value at a grid point uninterpolated.
  ///
  /// # Parameters
  ///
  /// * `point` - The position as a `[usize; 3]`
  ///
  /// # Returns
  ///
  /// A `Result` that is either
  ///   - OK atlas sample value at grid point
  ///   - Err An `InterpolaterError`
  ///
  fn get_atlas_sample(&self, point: [usize; 3]) -> Result<Option<U>, InterpolaterError>;

  ///
  /// Get the interpolated atlas value
  ///
  /// This gets the interpolated atlas value the `point` must be in bounds.
  ///
  /// In most cases this will simply be nearest neighbour as atlas values
  /// cannot typically be interpolated
  ///
  /// # Parameters
  ///
  /// * `point` - The point to get the interpolated atlas value as `[f32; 3]`
  ///
  /// # Returns
  ///
  /// A `Result` that is either
  ///   - OK interpolated atlas value at point
  ///   - Err An `InterpolaterError`
  ///
  fn get_atlas_value(&self, point: [usize; 3]) -> Result<Option<U>, InterpolaterError>;
}
