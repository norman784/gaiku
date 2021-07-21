use super::{Interpolater, InterpolaterError};
use crate::boundary::Boundary;

const EPSILON: f32 = 1e-4;

///
/// This interpolator simply returns the sample nearest
/// the requested point.
///
pub struct NearestNeighbour {
  data: Vec<f32>,
  atlas_data: Vec<u8>,
  dimensions: [usize; 3],
}

impl Interpolater<f32, u8> for NearestNeighbour {
  ///
  /// Creates a NearestNeighbour with both interpolated values and atlases
  ///
  /// This interpolator simple returns the sample data nearest the specified
  /// point.
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
  /// returns a NearestNeighbour
  ///
  /// # Examples
  ///
  /// ```
  /// # extern crate gaiku;
  /// # extern crate rand;
  /// use gaiku::common::prelude::*;
  /// use rand::Rng;
  /// let mut rng = rand::thread_rng();
  ///
  /// let samples: Vec<f32> = (0..27).map(|_| rng.gen_range((-10.)..(10.))).collect();
  /// let atlas_samples: Vec<u8> = (0..27).map(|_| rng.gen_range(0..10)).collect();
  /// let interpolator = NearestNeighbour::from_array_with_atlas(
  ///     &samples,
  ///     &atlas_samples,
  ///     3,
  ///     3,
  ///     3,
  /// );
  /// ```
  ///
  fn from_array_with_atlas(
    data: &[f32],
    atlas_data: &[u8],
    width: usize,
    height: usize,
    depth: usize,
  ) -> Self {
    Self {
      data: data.to_vec(),
      atlas_data: atlas_data.to_vec(),
      dimensions: [width, height, depth],
    }
  }

  ///
  /// Get's the boundary of the interpolation
  ///
  /// Interpolation cannot return values outside
  /// of its sample range. This method will
  /// inform you what those boundaries are.
  ///
  /// # Returns
  ///
  /// returns a Boundary bouding the sample data
  ///
  /// # Examples
  ///
  /// ```
  /// # extern crate gaiku;
  /// # extern crate rand;
  /// # use gaiku::common::prelude::*;
  /// # use rand::Rng;
  /// # let mut rng = rand::thread_rng();
  /// # let samples: Vec<f32> = (0..27).map(|_| rng.gen_range((-10.)..(10.))).collect();
  /// # let atlas_samples: Vec<u8> = (0..27).map(|_| rng.gen_range(0..10)).collect();
  /// # let interpolator = NearestNeighbour::from_array_with_atlas(
  /// #     &samples,
  /// #     &atlas_samples,
  /// #     3,
  /// #     3,
  /// #     3,
  /// # );
  /// let boundary = interpolator.get_boundary();
  /// assert!(boundary.contains(&[1., 1., 2.].into()));
  /// ```
  ///
  fn get_boundary(&self) -> Boundary {
    Boundary::new(
      &[-EPSILON, -EPSILON, -EPSILON].into(),
      &[
        self.dimensions[0] as f32 + EPSILON,
        self.dimensions[1] as f32 + EPSILON,
        self.dimensions[2] as f32 + EPSILON,
      ]
      .into(),
    )
  }

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
  fn get_sample(&self, point: [usize; 3]) -> Result<f32, InterpolaterError> {
    let idx =
      point[0] + point[1] * self.dimensions[0] + point[2] * self.dimensions[0] * self.dimensions[1];
    Ok(self.data[idx])
  }

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
  /// # Examples
  ///
  /// ```
  /// # extern crate gaiku;
  /// # extern crate rand;
  /// # use gaiku::common::prelude::*;
  /// # use rand::Rng;
  /// # let mut rng = rand::thread_rng();
  /// # let samples: Vec<f32> = (0..27).map(|_| rng.gen_range((-10.)..(10.))).collect();
  /// # let atlas_samples: Vec<u8> = (0..27).map(|_| rng.gen_range(0..10)).collect();
  /// # let interpolator = NearestNeighbour::from_array_with_atlas(
  /// #     &samples,
  /// #     &atlas_samples,
  /// #     3,
  /// #     3,
  /// #     3,
  /// # );
  /// let value = interpolator.get_value([0.25, 0.5, 1.]);
  /// ```
  ///
  fn get_value(&self, point: [f32; 3]) -> Result<f32, InterpolaterError> {
    let boundary = self.get_boundary();
    if !boundary.contains(&point.into()) {
      return Err(InterpolaterError::OutOfBounds {
        bounds: boundary,
        point,
      });
    }

    // Nearest point can be found by shifting the coordinates
    let i = (point[0] + 0.5) as usize;
    let j = (point[1] + 0.5) as usize;
    let k = (point[2] + 0.5) as usize;

    self.get_sample([i, j, k])
  }

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
  fn get_atlas_sample(&self, point: [usize; 3]) -> Result<Option<u8>, InterpolaterError> {
    let idx =
      point[0] + point[1] * self.dimensions[0] + point[2] * self.dimensions[0] * self.dimensions[1];
    Ok(self.atlas_data.get(idx).copied())
  }

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
  /// # Examples
  ///
  /// ```
  /// # #[macro_use] extern crate assert_matches;
  /// # extern crate gaiku;
  /// # extern crate rand;
  /// # use gaiku::common::prelude::*;
  /// # use rand::Rng;
  /// # let mut rng = rand::thread_rng();
  /// # let samples: Vec<f32> = (0..27).map(|_| rng.gen_range((-10.)..(10.))).collect();
  /// # let atlas_samples: Vec<u8> = (0..27).map(|_| rng.gen_range(0..10)).collect();
  /// # let interpolator = NearestNeighbour::from_array_with_atlas(
  /// #     &samples,
  /// #     &atlas_samples,
  /// #     3,
  /// #     3,
  /// #     3,
  /// # );
  /// let atlas_value = interpolator.get_atlas_value([0.25, 0.5, 1.]);
  /// assert_matches!(atlas_value, Ok(Some(_)));
  /// ```
  fn get_atlas_value(&self, point: [f32; 3]) -> Result<Option<u8>, InterpolaterError> {
    let boundary = self.get_boundary();
    if !boundary.contains(&point.into()) {
      return Err(InterpolaterError::OutOfBounds {
        bounds: boundary,
        point,
      });
    }

    // Nearest point can be found by shifting the coordinates
    let i = (point[0] + 0.5) as usize;
    let j = (point[1] + 0.5) as usize;
    let k = (point[2] + 0.5) as usize;

    self.get_atlas_sample([i, j, k])
  }
}
