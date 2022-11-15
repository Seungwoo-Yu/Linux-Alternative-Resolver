use crate::models::alt_config::AltConfig;
use crate::models::errors::error_combo::IOParseAlternativeResolveError;

pub trait AltConfigPersistence {
    fn resolve(&self) -> Result<AltConfig, IOParseAlternativeResolveError>;
    fn update(&self, config: &AltConfig) -> Result<(), IOParseAlternativeResolveError>;
}
