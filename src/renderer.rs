use crate::prelude::{Bounds, Triangle, VertexComponent, Vertex, ContainsPoint};

pub const CLEAR_CHARACTER_INDEX: u8 = 0;
// See: https://paulbourke.net/dataformats/asciiart/
pub const GRAYSCALE_CHARACTER_RANGE: &str = " .:-=+*#%@";

pub type Texture = Vec<Vec<u8>>;

#[derive(Debug, Default)]
pub enum DepthTestMode {
    #[default]
    DepthTestNone,
    DepthTest,
}

pub struct Renderer {
    texture: Texture,
    depth_symbols: Vec<char>,
    depth_test_mode: DepthTestMode,
    bounds: Bounds,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new_from_bounds(bounds: &Bounds) -> Self {
        let (width, height) = bounds.size();
        let texture = vec![vec![CLEAR_CHARACTER_INDEX; width]; height];

        println!("{}", texture.len());

        Self {
            texture,
            width,
            height,
            bounds: *bounds,
            ..Default::default()
        }
    }

    pub fn set_depth_test_mode(&mut self, mode: DepthTestMode) {
        self.depth_test_mode = mode;
    }

    pub fn clear(&mut self) {
        for line in self.texture.iter_mut() {
            for pixel in line.iter_mut() {
                *pixel = CLEAR_CHARACTER_INDEX;
            }
        }
    }

    #[allow(clippy::unnecessary_cast)]
    pub fn draw_triangle(&mut self, triangle: &Triangle) {
        for y in 0..=self.height {
            for x in 0..=self.width {
                let point = Vertex::new(self.bounds.min.x + x as VertexComponent, self.bounds.min.y + y as VertexComponent);
                if let Some(mut depth) = triangle.contains_point_with_depth(&point) {
                    depth = self.bounds.normalized_depth(depth);

                    let index = (depth * (self.depth_symbols.len() - 1) as f64).round() as usize;
                    // index 0 is whitespace, increase by 1, while making sure we do not exceed bounds
                    let index =  u8::min(1 + index as u8, self.depth_symbols.len() as u8 - 1);
                    match self.depth_test_mode {
                        DepthTestMode::DepthTestNone => {},
                        DepthTestMode::DepthTest => {
                            if self.texture[y][x] >= index {
                                continue;
                            }
                        }
                    }
                    self.texture[y][x] = index;
                }
            }
        }
    }

    pub fn render(&self) {
        for line in &self.texture {
            for pixel in line {
                print!("{}", self.depth_symbols.get(*pixel as usize).unwrap_or(&'?'))
            }
            println!()
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        let depth_symbols: Vec<_> =  GRAYSCALE_CHARACTER_RANGE.chars().collect();
        Self { texture: Default::default(), depth_symbols, width: 1, height: 1, bounds: Default::default(), depth_test_mode: Default::default() }
    }
}