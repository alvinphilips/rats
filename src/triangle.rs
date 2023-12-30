use crate::{
    bounds::{Bounds, GetBounds},
    line::Line,
    vertex::{ContainsPoint, Vertex},
};

#[derive(Debug)]
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

impl Triangle {
    #[allow(clippy::unnecessary_cast)]
    fn get_point_weights(&self, point: &Vertex) -> (f64, f64) {
        let a = &self.0;
        let b = &self.1;
        let c = &self.2;

        // TODO: clean up, possibly cache?
        let w1 = (a.y * (c.x - a.x) - a.x * (c.y - a.y) + point.x * (c.y - a.y)
            - point.y * (c.x - a.x)) as f64
            / ((b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)) as f64;
        let w2 = ((point.x - a.x) as f64 - w1 * (b.x - a.x) as f64) / (c.x - a.x) as f64;

        (w1, w2)
    }

    pub fn get_lines(&self) -> [Line; 3] {
        [
            Line(self.0, self.1),
            Line(self.1, self.2),
            Line(self.2, self.0),
        ]
    }
}

impl ContainsPoint for Triangle {
    fn contains_point(&self, point: &Vertex) -> bool {
        let (w1, w2) = self.get_point_weights(point);

        w1 >= 0. && w2 >= 0. && w1 + w2 <= 1.
    }

    #[allow(clippy::unnecessary_cast)]
    fn contains_point_with_depth(&self, point: &Vertex) -> Option<f64> {
        let (w1, w2) = self.get_point_weights(point);
        if w1 < 0. || w2 < 0. || w1 + w2 > 1. {
            return None;
        }
        let depth =
            self.0.z as f64 + w1 * (self.1.z - self.0.z) as f64 + w2 * (self.2.z - self.0.z) as f64;
        Some(depth)
    }
}

impl GetBounds for Triangle {
    fn get_bounds(&self) -> Bounds {
        let min = self.0.min(self.1).min(self.2);
        let max = self.0.max(self.1).max(self.2);

        Bounds { min, max }
    }
}
