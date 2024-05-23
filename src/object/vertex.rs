#[derive(Debug)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, w_opt: Option<f32>) -> Self {
        let w = w_opt.unwrap_or(1.0);
        Vertex { x, y, z, w }
    }
}
