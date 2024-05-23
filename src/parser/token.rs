use std::io;
use crate::error;

#[derive(Debug, Clone)]
pub enum Token {
    Vertex,
    VertexTexture,
    VertexNormal,
    Face,
    SmoothingGroup,
    ObjectName,
    MaterailName,
    MaterialLibrary,
    Comment,
    Value(String),
}

impl Token {
    fn from(s: &str) -> Self {
        match s {
            "v" => Self::Vertex,
            "vt" => Self::VertexTexture,
            "vn" => Self::VertexNormal,
            "f" => Self::Face,
            "s" => Self::SmoothingGroup,
            "o" => Self::ObjectName,
            "usemtl" => Self::MaterailName,
            "mtllib" => Self::MaterialLibrary,
            "#" => Self::Comment,
            _ => Self::Value(s.to_string()),
        }
    }

    pub fn lex(line: &str) -> Vec<Token> {
        line.split_whitespace()
            .map(Token::from)
            .collect()
    }

    pub fn extract_value(&self) -> Result<&String, io::Error> {
        match self {
            Token::Value(str) => Ok(str),
            _ => Err(error::non_value_token()),
        }
    }

    pub fn parse_f32(&self) -> Result<f32, io::Error> {
        let res = self.extract_value()?
            .parse::<f32>()
            .map_err(error::custom)?;

        Ok(res)
    }
}

pub trait RemoveComment {
    fn remove_comment(&mut self);
}

impl RemoveComment for Vec<Token> {
    fn remove_comment(&mut self) {
        if let Some(comment_pos) = self
            .iter()
            .position(|token| matches!(token, Token::Comment))
        {
            self.truncate(comment_pos);
        }
    }
}

pub trait ParseOptional {
    fn parse_optional_f32(&mut self) -> Result<Option<f32>, io::Error>;
}

impl ParseOptional for Option<&Token> {
    fn parse_optional_f32(&mut self) -> Result<Option<f32>, io::Error> {
        let res = match self {
            Some(token) => Some(token.parse_f32()?),
            None => None,
        };
        Ok(res)
    }
}
