use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;

pub trait ItemRegister {
    fn master_registered(&self, master_name: &String) -> Result<bool, IOParseAlternativeResolveError>;
    fn register_master(&self, master_name: &String) -> Result<(), IOParseAlternativeResolveError>;
    fn unregister_master(&self, master_name: &String) -> Result<(), IOParseAlternativeResolveError>;
    fn register_slaves(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn unregister_slaves(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn register(&self) -> Result<(), IOParseAlternativeResolveError>;
    fn unregister(&self) -> Result<(), IOParseAlternativeResolveError>;
}