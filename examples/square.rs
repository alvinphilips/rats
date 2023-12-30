use ratser::prelude::*;

fn main() {
    let triangles = vec![
        Triangle(
            Vertex::new(0., 0.),
            Vertex::new(0., 10.),
            Vertex::new(22., 10.),
        ),
        Triangle(
            Vertex::new(0., 0.),
            Vertex::new(22., 10.),
            Vertex::new(22., 0.),
        ),
    ];

    let mut bounds = triangles.get_bounds().pad(5.);

    // Hack to manually alter the character the shapes are drawn with :P
    bounds.min.z = -2.;
    bounds.max.z = 1.;

    let mut renderer = Renderer::new_from_bounds(&bounds);

    renderer.clear();

    renderer.draw_triangles(&triangles);

    renderer.render();
}
