use std::error::Error;
use crate::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for IOParseAlternativeResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            IOParseAlternativeResolveError::IOError(value) => value.fmt(f),
            IOParseAlternativeResolveError::ParseIntError(value) => value.fmt(f),
            IOParseAlternativeResolveError::AlternativeResolveError(value) => value.fmt(f),
        }
    }
}

impl Error for IOParseAlternativeResolveError {}
