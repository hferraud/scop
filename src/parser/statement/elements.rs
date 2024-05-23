use std::io;
use crate::error;
use crate::object::{Object, Face};
use crate::parser::Token;

const F_MIN_STATEMENT_LENGTH: usize = 3;
const F_MAX_PARAMETER_LENGTH: usize = 3;
const F_MIN_PARAMETER_LENGTH: usize = 1;

impl Face {
    pub fn parse(tokens: Vec<Token>, object: & mut Object) -> Result<(), io::Error> {
        let mut face = Self::new();
        if tokens.len() < F_MIN_STATEMENT_LENGTH {
            return Err(error::invalid_argument_length(
                format!(">= {F_MIN_STATEMENT_LENGTH}"),
                tokens.len(),
            ))
        }
        for token in tokens {
            face.argument_parse(token, object)?;
        }
        //TODO: Check if the vectors are of the same len
        object.add_face(face);
        Ok(())
    }

    pub fn argument_parse(&mut self, token: Token, object: & mut Object) -> Result<(), io::Error> {
        let parameters: Vec<String> = token.extract_value()?
            .split('/')
            .map(|s| s.to_string())
            .collect();
        let range = F_MIN_PARAMETER_LENGTH..=F_MAX_PARAMETER_LENGTH;
        if !range.contains(&parameters.len()) {
            return Err(error::invalid_argument_length(
                format!("{F_MIN_PARAMETER_LENGTH}..{F_MAX_PARAMETER_LENGTH}"),
                parameters.len(),
            ))
        }
        let v = parse_usize(&parameters[0])?;
        let vt = parse_optional_usize(parameters.get(1))?;
        let vn = parse_optional_usize(parameters.get(2))?;
        self.push_arg(v, vt, vn, object)?;
        Ok(())
    }
}

pub fn parse_usize(str: &String) -> Result<Option<usize>, io::Error> {
    match str.is_empty() {
        true => Ok(None),
        false => Ok(Some(str.parse().map_err(|e| error::custom(e))?))
    }
}

pub fn parse_optional_usize(opt: Option<&String>) -> Result<Option<usize>, io::Error> {
    let res = match opt {
        Some(str) => parse_usize(str)?,
        None => None,
    };
    Ok(res)
}
