pub mod prelude;
pub mod vertex;
pub mod line;
pub mod triangle;
pub mod bounds;

use crate::prelude::*;

fn main() {
    let triangle = Triangle(
        Vertex::new_with_depth(70., 50., -43.),
        Vertex::new_with_depth(-21., 10., 4.),
        Vertex::new_with_depth(50., 5., 14.),
    );
    let bounds = triangle.get_bounds().pad(3.);
    let depth_symbols: Vec<_> =  ".:-=+*#%@".chars().collect();

    let (width, height) = bounds.size();
    for y in 0..=height {
        for x in 0..=width {
            let point = Vertex::new(bounds.min.x + x as vertex::VertexComponent, bounds.min.y + y as vertex::VertexComponent);
            if let Some(mut depth) = triangle.contains_point_with_depth(&point) {
                depth = bounds.normalized_depth(depth);
                debug_assert!(depth >= 0. && depth <= 1.);
                let index = (depth * (depth_symbols.len() - 1) as f64) as usize;
                
                print!("{}", depth_symbols.get(index).unwrap())
            } else { 
                print!(" ")
            }
        }
        println!()
    }
    println!("{:?}", bounds);
}
