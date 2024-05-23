#[derive(Debug)]
pub struct VertexTexture {
    u: f32,
    v: Option<f32>,
    w: Option<f32>,
}

impl VertexTexture {
    pub fn new(u: f32, v: Option<f32>, w: Option<f32>) -> Self {
        VertexTexture { u, v, w }
    }
}
