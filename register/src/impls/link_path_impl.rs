#![cfg(target_os = "linux")]

use std::fs::remove_file;
use std::io::ErrorKind;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::link_path::LinkPath;
use crate::{ETC_ALTERNATIVE_PATH_STRING};
use crate::traits::path_register::PathRegister;

impl PathRegister for LinkPath {
    fn registered(&self) -> bool {
        !(PathBuf::from(&self.target_path).read_link().is_ok() &&
            PathBuf::from(ETC_ALTERNATIVE_PATH_STRING).join(&self.name).read_link().is_ok())
    }

    fn register(&self) -> Result<(), IOParseAlternativeResolveError> {
        let alternative_path = PathBuf::from(&self.alternative_path);
        let target_path = PathBuf::from(&self.target_path);
        let etc_path = PathBuf::from(ETC_ALTERNATIVE_PATH_STRING).join(&self.name);

        match symlink(&alternative_path, &etc_path) {
            Ok(_) => {},
            Err(error) => {
                return Err(IOParseAlternativeResolveError::IOError(error));
            }
        };

        match symlink(&etc_path, &target_path) {
            Ok(_) => {},
            Err(error) => {
                return Err(IOParseAlternativeResolveError::IOError(error));
            }
        };

        Ok(())
    }

    fn unregister(&self) -> Result<(), IOParseAlternativeResolveError> {
        let target_path = PathBuf::from(&self.target_path);
        let etc_path = PathBuf::from(ETC_ALTERNATIVE_PATH_STRING).join(&self.name);

        remove_if_exist(&target_path)?;
        remove_if_exist(&etc_path)
    }
}

fn remove_if_exist(path: &Path) -> Result<(), IOParseAlternativeResolveError> {
    match remove_file(path) {
        Ok(_) => {},
        Err(error) => {
            if error.kind() != ErrorKind::NotFound {
                return Err(IOParseAlternativeResolveError::IOError(error));
            }
        }
    };

    Ok(())
}
