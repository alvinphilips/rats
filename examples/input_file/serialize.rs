use rats::{Triangle, Vertex, VertexComponent};

/// Simple example trait that can deserialize a Type from a string slice
pub trait Serialize {
    fn get_from_str(input: &str) -> Self;
}

pub type TriangleList = Vec<Triangle>;

impl Serialize for Vertex {
    fn get_from_str(input: &str) -> Self {
        let components: Vec<VertexComponent> = input.split(',').filter_map(|val| val.parse::<VertexComponent>().ok()).collect();
        match components.len() {
            0 => Vertex::default(),
            1 => Vertex::new(components[0], components[0]),
            2 => Vertex::new(components[0], components[1]),
            3 => Vertex::new_with_depth(components[0], components[1], components[2]),
            _ => unreachable!()
        }
    }
}

impl Serialize for Triangle {
    fn get_from_str(input: &str) -> Self {
        let vertices = input.split(';').take(3).map(Vertex::get_from_str).collect::<Vec<_>>();
        assert!(vertices.len() == 3);
        Triangle(vertices[0], vertices[1], vertices[2])
    }
}

impl Serialize for TriangleList {
    fn get_from_str(input: &str) -> Self {
        input.lines().map(Triangle::get_from_str).collect()
    }
}