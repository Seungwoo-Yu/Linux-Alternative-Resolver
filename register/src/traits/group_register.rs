use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;

pub trait GroupRegister {
    fn register_master(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn unregister_master(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn register_slaves(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn unregister_slaves(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn register(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn unregister(&self) -> Result<(), IOParseAlternativeResolveError>;
}