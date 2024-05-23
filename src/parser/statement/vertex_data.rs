use std::io;
use crate::error;
use crate::object::{Object, Vertex, VertexNormal, VertexTexture};
use crate::parser::Token;
use crate::parser::token::ParseOptional;

const V_MIN_STATEMENT_LENGTH: usize = 3;
const V_MAX_STATEMENT_LENGTH: usize = 4;
const VN_STATEMENT_LENGTH: usize = 3;
const VT_MIN_STATEMENT_LENGTH: usize = 1;
const VT_MAX_STATEMENT_LENGTH: usize = 3;

impl Vertex {
    pub fn parse(tokens: Vec<Token>, object: &mut Object) -> Result<(), io::Error> {
        let range = V_MIN_STATEMENT_LENGTH..V_MAX_STATEMENT_LENGTH;
        if !range.contains(&tokens.len()) {
            return Err(error::invalid_argument_length(
                format!("{V_MIN_STATEMENT_LENGTH}..{V_MAX_STATEMENT_LENGTH}"),
                tokens.len(),
            ))
        }
        let x = tokens[0].parse_f32()?;
        let y = tokens[1].parse_f32()?;
        let z = tokens[2].parse_f32()?;
        let w = tokens.get(3).parse_optional_f32()?;
        object.add_vertex(Vertex::new(x, y, z, w));
        Ok(())
    }
}

impl VertexNormal {
    pub fn parse(tokens: Vec<Token>, object: &mut Object) -> Result<(), io::Error> {
        if tokens.len() != VN_STATEMENT_LENGTH {
            return Err(error::invalid_argument_length(
                format!("{VN_STATEMENT_LENGTH}"),
                tokens.len(),
            ))
        }
        let i = tokens[0].parse_f32()?;
        let j = tokens[1].parse_f32()?;
        let k = tokens[2].parse_f32()?;
        object.add_vertex_normal(VertexNormal::new(i, j, k));
        Ok(())
    }
}

impl VertexTexture {
    pub fn parse(tokens: Vec<Token>, object: &mut Object) -> Result<(), io::Error> {
        let range = VT_MIN_STATEMENT_LENGTH..=VT_MAX_STATEMENT_LENGTH;
        if !range.contains(&tokens.len()) {
            return Err(error::invalid_argument_length(
                format!("{VT_MIN_STATEMENT_LENGTH}..{VT_MAX_STATEMENT_LENGTH}"),
                tokens.len(),
            ))
        }
        let u = tokens[0].parse_f32()?;
        let v = tokens.get(1).parse_optional_f32()?;
        let w = tokens.get(2).parse_optional_f32()?;
        object.add_vertex_texture(VertexTexture::new(u, v, w));
        Ok(())
    }
}
