use crate::common_models::models::errors::error::AlternativeResolveError;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum IOParseAlternativeResolveError {
    IOError(io::Error),
    ParseIntError(ParseIntError),
    AlternativeResolveError(AlternativeResolveError),
}
