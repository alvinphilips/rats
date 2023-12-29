use rats::{prelude::*, renderer};

fn main() {
    let triangle = Triangle(
        Vertex::new_with_depth(0., 0., 0.),
        Vertex::new_with_depth(20., -20., 5.),
        Vertex::new_with_depth(40., 0., -5.),
    );
    let bounds = triangle.get_bounds().pad(10.);
    let mut renderer = Renderer::new_from_bounds(&bounds);

    renderer.draw_triangle(&triangle);
    renderer.render();
}