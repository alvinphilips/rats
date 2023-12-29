use crate::prelude::{Vertex, ContainsPoint, VertexComponent};

pub trait GetBounds {
    fn get_bounds(&self) -> Bounds;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Bounds {
    pub min: Vertex,
    pub max: Vertex,
}

impl Bounds {
    #[allow(dead_code)]
    pub fn pad(&mut self, padding: VertexComponent) -> Self {
        let (min_z, max_z) = (self.min.z, self.max.z);
        self.min -= padding;
        self.max += padding;

        // Reset Z values
        // TODO: Could possibly create a new Vertex with a zeroed out Z value and use that instead.
        self.min.z = min_z;
        self.max.z = max_z;

        *self
    }

    /// Size of the [`Bounds`], represented as a tuple of two [`usize`]s, denoting the width and height.
    /// 
    /// The `z` component is ignored.
    #[allow(dead_code)]
    pub fn size(&self) -> (usize, usize) {
        let width = (self.max.x - self.min.x) as usize;
        let height = (self.max.y - self.min.y) as usize;

        (width, height)
    }

    #[allow(dead_code)]
    #[allow(clippy::unnecessary_cast)]
    pub fn normalized_depth(&self, depth: f64) -> f64 {
        let z_size = self.max.z - self.min.z;

        // Cannot normalize value :(
        if z_size == VertexComponent::default() {
            return 0.
        }

        let min_depth = depth - self.min.z as f64;

        min_depth / z_size as f64
    }
}

impl core::ops::Add for Bounds {
    type Output = Bounds;

    /// 'Adding' two [`Bounds`] consists of calculating the union of the two.
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max)
        }
    }
}

impl core::ops::AddAssign for Bounds {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ContainsPoint for Bounds {
    fn contains_point(&self, point: &Vertex) -> bool {
        // Check if any of our component values are smaller than the Bound's minimum
        if point.x < self.min.x || point.y < self.min.y || point.z < self.min.z {
            return false;
        }

        // Check if any of our component values are larger than the Bound's maximum
        if point.x > self.max.x || point.y > self.max.y || point.z > self.max.z {
            return false;
        }

        // Bounds checking passed, we're inside bounds
        true
    }
}

impl GetBounds for Bounds {
    fn get_bounds(&self) -> Bounds {
        *self
    }
}