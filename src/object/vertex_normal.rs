#[derive(Debug)]
pub struct VertexNormal {
    i: f32,
    j: f32,
    k: f32,
}

impl VertexNormal {
    pub fn new(i: f32, j: f32, k: f32) -> Self {
        VertexNormal { i, j, k }
    }
}
