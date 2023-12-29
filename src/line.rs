use crate::prelude::{Vertex, ContainsPoint, GetBounds, Bounds};

pub const POINT_IN_LINE_THRESHOLD: f64 = 0.5;

/// A Line (technically a line segment) contains 
#[derive(Debug, Clone, Copy)]
pub struct Line(pub Vertex, pub Vertex);

impl Line {
    #[allow(dead_code)]
    pub fn new(start: Vertex, end: Vertex) -> Self {
        Self(start, end)
    }

    pub fn contains_point_with_threshold(&self, point: &Vertex, threshold: f64) -> bool {
        // TODO: possible division by zero if line is vertical, i.e. x values are the same
        let slope = ((self.1.y - self.0.y) as f64) / (self.1.x - self.0.x) as f64;
        let b = self.0.y as f64 - slope * self.0.x as f64;

        f64::abs(point.x as f64 * slope + b - point.y as f64) < threshold
    }
}

impl ContainsPoint for Line {
    fn contains_point(&self, point: &Vertex) -> bool {
        self.contains_point_with_threshold(point, POINT_IN_LINE_THRESHOLD)
    }
    fn contains_point_with_depth(&self, point: &Vertex) -> Option<f64> {
        if !self.contains_point(point) {
            return None;
        }

        todo!()
    }
}

impl GetBounds for Line {
    fn get_bounds(&self) -> Bounds {
        let min = self.0.min(self.1);
        let max = self.0.max(self.1);

        Bounds { min, max }
    }
}