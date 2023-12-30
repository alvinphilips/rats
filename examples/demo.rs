use ratser::prelude::*;

fn main() {
    let triangle = Triangle(
        Vertex::new_with_depth(70., 50., -43.),
        Vertex::new_with_depth(-21., 10., 4.),
        Vertex::new_with_depth(50., 5., 14.),
    );
    let new_triangle = Triangle(
        Vertex::new_with_depth(20., 50., 23.),
        Vertex::new_with_depth(-10., 5., -4.),
        Vertex::new_with_depth(60., 32., 10.),
    );

    let mut bounds = triangle.get_bounds();
    bounds += new_triangle.get_bounds();
    bounds.pad(5.);

    let mut renderer = Renderer::new_from_bounds(&bounds);

    renderer.set_depth_test_mode(DepthTestMode::DepthTest);
    renderer.clear();
    renderer.draw_triangle(&triangle);
    renderer.draw_triangle(&new_triangle);
    renderer.render();

    println!("{:?}", bounds);
}
