use super::{Interpolater, InterpolaterError};
use crate::boundary::Boundary;
use glam::Vec3;

const EPSILON: f32 = 1e-4;

///
/// This interpolator simply returns the sample nearest
/// the requested point.
///
pub struct TriLinear {
  data: Vec<f32>,
  atlas_data: Vec<u8>,
  dimensions: [usize; 3],
}

///
/// This interpolator uses linear interpolation for the samples
/// and nearest neighbour for the atlas values.
///
impl Interpolater<f32, u8> for TriLinear {
  ///
  /// Creates a TriLinear with both interpolated values and atlases
  ///
  /// This interpolator will use linear interpolation on 3d
  /// for the samples.
  ///
  /// It will use nearestneighbour for the atlas samples.
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
  /// returns a TriLinear
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::prelude::*
  /// use rand::Rng;
  /// let mut rng = rand::thread_rng();
  ///
  /// let samples: Vec<f32> = (0..27).map(|_| rng.gen_range(-10., 10.)).collect();
  /// let atlas_samples: Vec<u8> = (0..27).map(|_| rng.gen_range(0, 10)).collect();
  /// let interpolator = TriLinear::from_array_with_atlas(
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
  /// #use crate::prelude::*
  /// #use rand::Rng;
  /// #let mut rng = rand::thread_rng();
  /// #let samples: Vec<f32> = (0..27).map(|_| rng.gen_range(-10., 10.)).collect();
  /// #let atlas_samples: Vec<u8> = (0..27).map(|_| rng.gen_range(0, 10)).collect();
  /// #let interpolator = TriLinear::from_array_with_atlas(
  /// #    &samples,
  /// #    &atlas_samples,
  /// #    3,
  /// #    3,
  /// #    3,
  /// #);
  /// let boundary = interpolator.get_boundary();
  /// assert!(boundary.contains([1., 1., 2.].into());
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
  /// #use crate::prelude::*
  /// #use rand::Rng;
  /// #let mut rng = rand::thread_rng();
  /// #let samples: Vec<f32> = (0..27).map(|_| rng.gen_range(-10., 10.)).collect();
  /// #let atlas_samples: Vec<u8> = (0..27).map(|_| rng.gen_range(0, 10)).collect();
  /// #let interpolator = TriLinear::from_array_with_atlas(
  /// #    &samples,
  /// #    &atlas_samples,
  /// #    3,
  /// #    3,
  /// #    3,
  /// #);
  /// let value = interpolator.get_value([0.25, 0.5, 1.]);
  /// ```
  ///
  fn get_value(&self, point: [f32; 3]) -> Result<f32, InterpolaterError> {
    let point: Vec3 = point.into();

    // Boundary check
    let boundary = self.get_boundary();
    if !boundary.contains(&point) {
      return Err(InterpolaterError::OutOfBounds {
        bounds: boundary,
        point: point.into(),
      });
    }

    // Ensure we are in range!
    // this should change any rounding error -0.0002 into 0.
    // that the above bounary check dosen't reject
    let point: Vec3 = [
      point[0].clamp(0., (self.dimensions[0] - 1) as f32),
      point[1].clamp(0., (self.dimensions[1] - 1) as f32),
      point[2].clamp(0., (self.dimensions[2] - 1) as f32),
    ]
    .into();

    // Grid cell can be found by flooring
    let grid_origin: Vec3 = point.floor();

    // Nearest neighbour can be found by a shift and a floor
    let nearest_neighbour = (point + Vec3::from([0.5, 0.5, 0.5])).floor();

    // Are we close enough to a nearest neighbour?
    if nearest_neighbour.distance(point) < EPSILON {
      return self.get_sample([
        nearest_neighbour[0] as usize,
        nearest_neighbour[1] as usize,
        nearest_neighbour[2] as usize,
      ]);
    }

    // Get the sample coords of the grid
    let i = grid_origin[0] as usize;
    let j = grid_origin[1] as usize;
    let k = grid_origin[2] as usize;

    let cell_coord = point - grid_origin;

    // There a few edge cases here
    //
    // we want a cell of 8 points at
    // 000
    // 100
    // 110
    // 010
    // 001
    // 101
    // 111
    // 011
    //
    // However some of those points will be out of bounds
    // at the edges
    //
    // In these cases we just want to do the 2d/1d Interpolation
    // rather than the 3d
    //

    let i_limit = self.dimensions[0] - 1;
    let j_limit = self.dimensions[1] - 1;
    let k_limit = self.dimensions[2] - 1;

    let result = if i == i_limit && j == j_limit && k == k_limit {
      // xyz maxima case (this should be caught by the above check anyways)
      let f000 = self.get_sample([i, j, k])?;
      f000
    } else if i == i_limit && j == j_limit {
      // xy maxima case
      let f000 = self.get_sample([i, j, k])?;
      let f001 = self.get_sample([i, j, k + 1])?;
      let z_sample = cell_coord[2];
      f000 * (1. - z_sample) + f001 * z_sample
    } else if i == i_limit && k == k_limit {
      // xz maxima case
      let f000 = self.get_sample([i, j, k])?;
      let f010 = self.get_sample([i, j + 1, k])?;
      let y_sample = cell_coord[1];
      f000 * (1. - y_sample) + f010 * (y_sample)
    } else if j == j_limit && k == k_limit {
      // yz maxima case
      let f000 = self.get_sample([i, j, k])?;
      let f100 = self.get_sample([i + 1, j, k])?;
      let x_sample = cell_coord[0];
      f000 * (1. - x_sample) + f100 * (x_sample)
    } else if i == i_limit {
      // x maxima case
      let f000 = self.get_sample([i, j, k])?;
      let f001 = self.get_sample([i, j, k + 1])?;
      let f010 = self.get_sample([i, j + 1, k])?;
      let f011 = self.get_sample([i, j + 1, k + 1])?;
      let y_sample = cell_coord[1];
      let z_sample = cell_coord[2];
      f000 * (1. - y_sample) * (1. - z_sample)
        + f001 * (1. - y_sample) * z_sample
        + f010 * (y_sample) * (1. - z_sample)
        + f011 * (y_sample) * (z_sample)
    } else if j == j_limit {
      // y maxima case
      let f000 = self.get_sample([i, j, k])?;
      let f001 = self.get_sample([i, j, k + 1])?;
      let f100 = self.get_sample([i + 1, j, k])?;
      let f101 = self.get_sample([i + 1, j, k + 1])?;
      let x_sample = cell_coord[0];
      let z_sample = cell_coord[2];
      f000 * (1. - x_sample) * (1. - z_sample)
        + f001 * (1. - x_sample) * z_sample
        + f100 * (x_sample) * (1. - z_sample)
        + f101 * (x_sample) * (z_sample)
    } else if k == k_limit {
      // z maxima case
      let f000 = self.get_sample([i, j, k])?;
      let f010 = self.get_sample([i, j + 1, k])?;
      let f100 = self.get_sample([i + 1, j, k])?;
      let f110 = self.get_sample([i + 1, j + 1, k])?;
      let x_sample = cell_coord[0];
      let y_sample = cell_coord[1];
      f000 * (1. - x_sample) * (1. - y_sample)
        + f010 * (1. - x_sample) * (y_sample)
        + f100 * (x_sample) * (1. - y_sample)
        + f110 * (x_sample) * (y_sample)
    } else if i < i_limit && j < j_limit && k < k_limit {
      // normal case 3d case
      let f000 = self.get_sample([i, j, k])?;
      let f001 = self.get_sample([i, j, k + 1])?;
      let f010 = self.get_sample([i, j + 1, k])?;
      let f100 = self.get_sample([i + 1, j, k])?;
      let f011 = self.get_sample([i, j + 1, k + 1])?;
      let f101 = self.get_sample([i + 1, j, k + 1])?;
      let f110 = self.get_sample([i + 1, j + 1, k])?;
      let f111 = self.get_sample([i + 1, j + 1, k + 1])?;
      let x_sample = cell_coord[0];
      let y_sample = cell_coord[1];
      let z_sample = cell_coord[2];
      f000 * (1. - x_sample) * (1. - y_sample) * (1. - z_sample)
        + f001 * (1. - x_sample) * (1. - y_sample) * z_sample
        + f010 * (1. - x_sample) * (y_sample) * (1. - z_sample)
        + f100 * (x_sample) * (1. - y_sample) * (1. - z_sample)
        + f011 * (1. - x_sample) * (y_sample) * (z_sample)
        + f101 * (x_sample) * (1. - y_sample) * (z_sample)
        + f110 * (x_sample) * (y_sample) * (1. - z_sample)
        + f111 * (x_sample) * (y_sample) * (z_sample)
    } else {
      return Err(InterpolaterError::OutOfBounds {
        point: point.into(),
        bounds: self.get_boundary(),
      });
    };

    Ok(result)
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
  /// ```
  /// #use crate::prelude::*
  /// #use rand::Rng;
  /// #let mut rng = rand::thread_rng();
  /// #let samples: Vec<f32> = (0..27).map(|_| rng.gen_range(-10., 10.)).collect();
  /// #let atlas_samples: Vec<u8> = (0..27).map(|_| rng.gen_range(0, 10)).collect();
  /// #let interpolator = TriLinear::from_array_with_atlas(
  /// #    &samples,
  /// #    &atlas_samples,
  /// #    3,
  /// #    3,
  /// #    3,
  /// #);
  /// let atlas_value = interpolator.get_atlas_value([0.25, 0.5, 1.]);
  /// assert_eq!(atlas_value, 1);
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
