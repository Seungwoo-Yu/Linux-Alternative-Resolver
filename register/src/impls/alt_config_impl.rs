#![cfg(target_os = "linux")]

use linux_alternative_resolver_shared::common_models::models::alt_config::AltConfig;
use linux_alternative_resolver_shared::common_models::models::errors::error::AlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::errors::error::Type::EmptyGroupListInConfig;
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use crate::traits::alt_config_register::AltConfigRegister;
use crate::traits::group_register::GroupRegister;

impl AltConfigRegister for AltConfig {
    fn register(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.alternatives.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: EmptyGroupListInConfig }
                )
            );
        }

        for value in self.alternatives.iter() {
            value.register()?;
        }

        Ok(())
    }

    fn unregister(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.alternatives.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: EmptyGroupListInConfig }
                )
            );
        }

        for value in self.alternatives.iter() {
            value.unregister()?;
        }

        Ok(())
    }
}