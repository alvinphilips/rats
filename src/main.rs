pub mod prelude;
pub mod vertex;
pub mod line;
pub mod triangle;
pub mod bounds;

use crate::prelude::*;

fn main() {
    let triangle = Triangle(Vertex::new_with_depth(5, 0, -2), Vertex::new(0, 10), Vertex::new(10, 10));
    let bounds = triangle.get_bounds().pad(1);
    let depth_symbols: Vec<_> =  ".:-=+*#%@".chars().collect();

    let (width, height) = bounds.size();
    for y in 0..=height {
        for x in 0..=width {
            let point = Vertex::new(bounds.min.x + x as i32, bounds.min.y + y as i32);
            if let Some(mut depth) = triangle.contains_point_with_depth(&point) {
                depth = bounds.normalized_depth(depth);

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
