mod vertex;
mod face;

pub use vertex::Vertex;
pub use face::Face;

pub struct Object {
    faces: Vec<Face>
}