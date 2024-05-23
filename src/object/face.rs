use std::io;
use std::rc::Rc;
use crate::error;

use crate::object::{Object, Vertex, VertexNormal, VertexTexture};

#[derive(Debug)]
pub struct Face {
    pub vertices: Vec<Rc<Vertex>>,
    pub vertices_texture: Vec<Rc<VertexTexture>>,
    pub vertices_normal: Vec<Rc<VertexNormal>>,
}

impl Face {
    pub fn new() -> Self {
        Face {
            vertices: vec![],
            vertices_texture: vec![],
            vertices_normal: vec![],
        }
    }

    pub fn push_arg(&mut self, v: Option<usize>, vt: Option<usize>, vn: Option<usize>, object: &Object) -> Result<(), io::Error> {
        Self::push(v, &mut self.vertices, &object.vertices)?;
        Self::push(vt, &mut self.vertices_texture, &object.vertices_texture)?;
        Self::push(vn, &mut self.vertices_normal, &object.vertices_normal)?;
        Ok(())
    }

    fn push<T>(opt: Option<usize>, face_vec: &mut Vec<Rc<T>>, obj_vec: &Vec<Rc<T>>) -> Result<(), io::Error> {
        if let Some(index) = opt {
            match obj_vec.get(index - 1) {
                Some(vertex) => face_vec.push(Rc::clone(vertex)),
                None => return Err(error::index_out_of_bound(index, obj_vec.len())),
            }
        }
        Ok(())
    }
}
