use crate::common_models::models::errors::error::{AlternativeResolveError, Type};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for AlternativeResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Alternative Resolve Error: {}", self.error_type)
    }
}

impl Error for AlternativeResolveError {}
