use linux_alternative_resolver_shared::common_models::models::alt_config::AltConfig;
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;

pub trait AltConfigPersistence {
    fn resolve(&self) -> Result<AltConfig, IOParseAlternativeResolveError>;
    fn update(&self, config: &AltConfig) -> Result<(), IOParseAlternativeResolveError>;
}
