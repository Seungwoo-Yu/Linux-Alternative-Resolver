#![cfg(target_os = "linux")]

use linux_alternative_resolver_shared::common_models::models::errors::error::AlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::errors::error::Type::{DifferentMasterPathWithName, MasterPathNotFound, SlavePathNotFound};
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::link_item::LinkItem;
use linux_alternative_resolver_shared::common_models::models::link_path::LinkPath;
use crate::traits::item_register::ItemRegister;
use crate::traits::path_register::PathRegister;

impl ItemRegister for LinkItem {
    fn master_registered(&self, master_name: &String) -> Result<bool, IOParseAlternativeResolveError> {
        Ok(master_path(self, master_name)?.registered())
    }

    fn register_master(&self, master_name: &String) -> Result<(), IOParseAlternativeResolveError> {
        Ok(master_path(self, master_name)?.register()?)
    }

    fn unregister_master(&self, master_name: &String) -> Result<(), IOParseAlternativeResolveError> {
        Ok(master_path(self, master_name)?.unregister()?)
    }

    fn register_slaves(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.paths.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: MasterPathNotFound }
                )
            )
        } else if self.paths.len() == 1 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: SlavePathNotFound }
                )
            )
        }

        let paths: Vec<&LinkPath> = self.paths.iter().collect();

        for value in (paths[1..]).iter() {
            value.register()?;
        }

        Ok(())
    }

    fn unregister_slaves(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.paths.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: MasterPathNotFound }
                )
            )
        } else if self.paths.len() == 1 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: SlavePathNotFound }
                )
            )
        }

        let paths: Vec<&LinkPath> = self.paths.iter().collect();

        for value in (paths[1..]).iter() {
            value.unregister()?;
        }

        Ok(())
    }

    fn register(&self) -> Result<(), IOParseAlternativeResolveError> {
        for value in self.paths.iter() {
            value.register()?;
        }

        Ok(())
    }

    fn unregister(&self) -> Result<(), IOParseAlternativeResolveError> {
        for value in self.paths.iter() {
            value.unregister()?;
        }

        Ok(())
    }
}

fn master_path<'t>(link_item: &'t LinkItem, master_name: &'t String) -> Result<&'t LinkPath, IOParseAlternativeResolveError> {
    let _master = link_item.paths.get_index(0);

    let master = match _master {
        None => {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: MasterPathNotFound }
                )
            );
        }
        Some(value) => value,
    };

    if !master.name.eq(master_name) {
        return Err(
            IOParseAlternativeResolveError::AlternativeResolveError(
                AlternativeResolveError { error_type: DifferentMasterPathWithName }
            )
        );
    }

    Ok(master)
}
