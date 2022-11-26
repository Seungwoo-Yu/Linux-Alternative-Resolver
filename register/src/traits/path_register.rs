use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;

pub trait PathRegister {
    fn registered(&self) -> bool;
    fn register(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn unregister(&self) -> Result<(), IOParseAlternativeResolveError>;
}
