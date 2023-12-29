pub type VertexComponent = f64;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vertex {
    pub x: VertexComponent,
    pub y: VertexComponent,
    pub z: VertexComponent,
}

impl Vertex {
    pub fn new(x: VertexComponent, y: VertexComponent) -> Self {
        Self::new_with_depth(x, y, Default::default())
    }

    pub fn new_with_depth(x: VertexComponent, y: VertexComponent, depth: VertexComponent) -> Self {
        Self {
            x,
            y,
            z: depth
        }
    }
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl core::cmp::PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        if self == other {
            return Some(Ordering::Equal)
        }
        if self.x <= other.x && self.y <= other.y && self.z <= other.z {
            return Some(Ordering::Less)
        }
        if self.x >= other.x && self.y >= other.y && self.z >= other.z {
            return Some(Ordering::Greater)
        }
        None
    }
}

impl core::ops::AddAssign for Vertex {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl core::ops::SubAssign for Vertex {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl core::ops::AddAssign<VertexComponent> for Vertex {
    fn add_assign(&mut self, rhs: VertexComponent) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl core::ops::SubAssign<VertexComponent> for Vertex {
    fn sub_assign(&mut self, rhs: VertexComponent) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

pub trait ContainsPoint {
    fn contains_point(&self, point: &Vertex) -> bool;
    fn contains_point_with_depth(&self, point: &Vertex) -> Option<f64> {
        if self.contains_point(point) {
            return Some(f64::NAN);
        }
        None
    }
}