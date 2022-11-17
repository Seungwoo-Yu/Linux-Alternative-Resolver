#![cfg(target_os = "linux")]

use linux_alternative_resolver_shared::common_models::models::errors::error::AlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::errors::error::Type::EmptyItemListInGroup;
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::link_group::LinkGroup;
use crate::traits::alt_config_register::AltConfigRegister;
use crate::traits::item_register::ItemRegister;

impl AltConfigRegister for LinkGroup {
    fn register(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.items.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: EmptyItemListInGroup }
                )
            );
        }

        for value in self.items.iter() {
            value.register()?;
        }

        Ok(())
    }

    fn unregister(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.items.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: EmptyItemListInGroup }
                )
            );
        }

        for value in self.items.iter() {
            value.register()?;
        }

        Ok(())
    }
}