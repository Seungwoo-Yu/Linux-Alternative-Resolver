#![cfg(target_os = "linux")]

use linux_alternative_resolver_shared::common_models::models::errors::error::AlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::errors::error::Type::{DifferentMasterPathWithName, EmptyItemListInGroup, MasterPathNotFound};
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::link_group::LinkGroup;
use linux_alternative_resolver_shared::common_models::models::link_item::LinkItem;
use linux_alternative_resolver_shared::common_models::models::link_path::LinkPath;
use crate::traits::group_register::GroupRegister;
use crate::traits::item_register::ItemRegister;
use crate::traits::path_register::PathRegister;

impl GroupRegister for LinkGroup {
    fn register_master(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.items.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: EmptyItemListInGroup }
                )
            );
        }

        for value in master_path(self, &self.name)?.iter() {
            value.register()?;
        }

        Ok(())
    }

    fn unregister_master(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.items.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: EmptyItemListInGroup }
                )
            );
        }

        for value in master_path(self, &self.name)?.iter() {
            value.unregister()?;
        }

        Ok(())
    }

    fn register_slaves(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.items.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: EmptyItemListInGroup }
                )
            );
        }

        let paths: Vec<&LinkItem> = self.items.iter().collect();

        for value in (paths[1..]).iter() {
            value.register_slaves()?;
        }

        Ok(())
    }

    fn unregister_slaves(&self) -> Result<(), IOParseAlternativeResolveError> {
        if self.items.len() == 0 {
            return Err(
                IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError { error_type: EmptyItemListInGroup }
                )
            );
        }

        let paths: Vec<&LinkItem> = self.items.iter().collect();

        for value in (paths[1..]).iter() {
            value.unregister_slaves()?;
        }

        Ok(())
    }

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

fn master_path<'t>(link_group: &'t LinkGroup, master_name: &'t String) -> Result<Vec<&'t LinkPath>, IOParseAlternativeResolveError> {
    let mut paths: Vec<&LinkPath> = vec![];

    for value in link_group.items.iter() {
        let _master = value.paths.get_index(0);

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

        paths.push(master);
    }

    Ok(paths)
}
