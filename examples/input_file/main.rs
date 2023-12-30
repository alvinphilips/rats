use rats::prelude::*;
mod serialize;

use serialize::*;

// INPUT DATA FORMAT:
// list of Triangles, separated by newlines. each Triangle has 3 vertices, separated by a semicolon (;), 
// and each Vertex consists of between 0 and 3 numbers, to denote the x and y values.
// 
// Vertex behaviour based on count of components provided:
// - 0 : empty Vertex, i.e. all components are zeroed out
// - 1 : Vertex with x and y components set to provided value, z zeroed out
// - 2 : Vertex with provided x amd y values, z zeroed out
// - 3 : x, y and z values as provided

fn main() {
    // Could use a file argument here, but I do not feel like adding file reads
    let triangles = TriangleList::get_from_str(include_str!("square.trianglelist"));

    let mut bounds = Bounds::default();
    for triangle in &triangles {
        bounds += triangle.get_bounds();
    }

    bounds.pad(5.);

    let mut renderer = Renderer::new_from_bounds(&bounds);

    renderer.clear();

    for triangle in &triangles {
        renderer.draw_triangle(triangle);
    }

    renderer.render();
}
