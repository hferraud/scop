mod face;
mod vertex;
mod vertex_normal;
mod vertex_texture;

use std::rc::Rc;

pub use face::Face;
pub use vertex::Vertex;
pub use vertex_normal::VertexNormal;
pub use vertex_texture::VertexTexture;

#[derive(Debug)]
pub struct Object {
    pub vertices: Vec<Rc<Vertex>>,
    pub vertices_normal: Vec<Rc<VertexNormal>>,
    pub vertices_texture: Vec<Rc<VertexTexture>>,
    pub faces: Vec<Face>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            vertices_normal: vec![],
            vertices_texture: vec![],
            faces: vec![],
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(Rc::new(vertex));
    }

    pub fn add_vertex_normal(&mut self, vertex_normal: VertexNormal) {
        self.vertices_normal.push(Rc::new(vertex_normal));
    }

    pub fn add_vertex_texture(&mut self, vertex_texture: VertexTexture) {
        self.vertices_texture.push(Rc::new(vertex_texture));
    }

    pub fn add_face(&mut self, face: Face) {
        self.faces.push(face);
    }
}