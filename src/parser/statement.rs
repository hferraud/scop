mod vertex_data;
mod elements;

use std::io;

use crate::error;
use crate::object::{Object, Vertex, VertexTexture, VertexNormal, Face};
use crate::parser::Token;

pub fn statement_router(mut tokens: Vec<Token>, object: &mut Object) -> Result<(), io::Error> {
    if tokens.is_empty() {
        return Ok(())
    }
    let identifier = tokens.remove(0);
    match identifier {
        Token::Value(_) => Err(error::invalid_token(&identifier)),
        Token::Vertex => Vertex::parse(tokens, object),
        Token::VertexTexture => VertexTexture::parse(tokens, object),
        Token::VertexNormal => VertexNormal::parse(tokens, object),
        Token::Face => Face::parse(tokens, object),
        Token::SmoothingGroup => Ok(()),
        Token::ObjectName => Ok(()),
        Token::MaterailName => Ok(()),
        Token::MaterialLibrary => Ok(()),
        _ => Err(error::unexpected_token(&identifier)),
    }
}

