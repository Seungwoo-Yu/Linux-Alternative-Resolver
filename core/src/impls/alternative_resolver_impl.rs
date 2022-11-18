use crate::alternative_resolver::AlternativeResolver;
use linux_alternative_resolver_shared::common_models::models::alt_config::AltConfig;
use linux_alternative_resolver_shared::common_models::models::errors::error::AlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::errors::error::Type::{
    ExecutionPathNotRecognized, FamilyPriorityNotRecognized, FirstEmptyLineNotRecognized,
    MasterPathNotRecognized, NoAvailableAltPath, TargetPathNotRecognized,
};
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use linux_alternative_resolver_shared::common_models::models::link_group::LinkGroup;
use linux_alternative_resolver_shared::common_models::models::link_item::LinkItem;
use linux_alternative_resolver_shared::common_models::models::link_path::LinkPath;
use crate::traits::alt_config_persistence::AltConfigPersistence;
use crate::{is_os_like, POSSIBLE_PATHS};
use indexmap::{IndexMap, IndexSet};
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{f64, fs, io, isize};

impl AltConfigPersistence for AlternativeResolver {
    fn resolve(&self) -> Result<AltConfig, IOParseAlternativeResolveError> {
        let path = match available_alt_path() {
            Ok(value) => value,
            Err(error) => {
                return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                    error,
                ));
            }
        };

        let child_paths = match flatten_read_dir(&path) {
            Ok(value) => value,
            Err(error) => {
                return Err(IOParseAlternativeResolveError::IOError(error));
            }
        };

        let _child_raw_strings: Result<Vec<(String, String)>, io::Error> = child_paths
            .iter()
            .map(|value| {
                Ok((
                    value.file_name().unwrap().to_string_lossy().to_string(),
                    fs::read_to_string(value)?
                ))
            })
            .collect();
        let child_raw_strings = match _child_raw_strings {
            Ok(value) => value,
            Err(error) => {
                return Err(IOParseAlternativeResolveError::IOError(error));
            }
        };

        convert_strings_to_alt_config(&child_raw_strings)
    }

    fn update(&self, config: &AltConfig) -> Result<(), IOParseAlternativeResolveError> {
        let alt_path = match available_alt_path() {
            Ok(value) => value,
            Err(error) => {
                return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                    error,
                ));
            }
        };

        let config_map = match convert_alt_config_to_hashmap(config) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };

        for (key, value) in (&config_map).iter() {
            match fs::write((&alt_path).join(key), value) {
                Ok(_) => {}
                Err(error) => {
                    return Err(IOParseAlternativeResolveError::IOError(error));
                }
            }
        }

        Ok(())
    }
}

fn available_alt_path() -> Result<PathBuf, AlternativeResolveError> {
    let _paths = POSSIBLE_PATHS;
    let _path = (&_paths)
        .iter()
        .map(|value| Path::new(value))
        .filter(|value| value.exists() && value.is_dir())
        .next();

    return match &_path {
        None => Err(AlternativeResolveError {
            error_type: NoAvailableAltPath,
        }),
        Some(value) => Ok(PathBuf::from(value)),
    };
}

fn flatten_read_dir(path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut result: Vec<PathBuf> = vec![];

    match path.read_dir() {
        Ok(value) => {
            for value2 in value {
                if let Ok(value3) = value2 {
                    let inner_path = value3.path();

                    if inner_path.is_dir() {
                        match flatten_read_dir(&inner_path) {
                            Ok(value) => {
                                for value in value {
                                    result.push(value);
                                }
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        }
                    } else {
                        result.push(inner_path);
                    }
                }
            }
        }
        Err(error) => {
            return Err(error);
        }
    }

    Ok(result)
}

pub fn convert_strings_to_alt_config(
    data: &Vec<(String, String)>,
) -> Result<AltConfig, IOParseAlternativeResolveError> {
    let mut result = AltConfig {
        alternatives: IndexSet::default(),
    };

    for (filename, value) in data.iter() {
        let lines: Vec<String> = value.lines().map(|value| value.to_string()).collect();

        // First line must be selection data
        let raw_auto_selection = (&lines)
            .get(0)
            .map(|value| (*value).to_string())
            .unwrap_or("auto".to_string());
        let auto_selection = (&raw_auto_selection).eq("auto");

        // Second line must be path of master
        let master_path = match (&lines).get(1).map(|value| (*value).to_string()) {
            None => {
                return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError {
                        error_type: MasterPathNotRecognized,
                    },
                ))
            }
            Some(value) => value,
        };
        let mut link_group = LinkGroup {
            name: match (&master_path).to_string()
                .split('/')
                .last()
                .or(Some(&master_path)) {
                None => {
                    return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                        AlternativeResolveError {
                            error_type: MasterPathNotRecognized,
                        },
                    ))
                }
                Some(value) => value,
            }.to_string(),
            filename: filename.to_string(),
            selected: None,
            items: IndexSet::default(),
        };

        // Find where slaves' data (or master's data in case of there is none but master only) ends
        let first_empty_line_pos = match (&lines).iter().position(|value| value.eq(&"")) {
            None => {
                return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError {
                        error_type: FirstEmptyLineNotRecognized,
                    },
                ))
            }
            Some(value) => value,
        };

        // HashMap where master and slave data puts into
        let mut targets: IndexMap<String, String> = IndexMap::from([(
            (&link_group.name).to_string(),
            match (&lines).get(1).map(|value| value.to_string()) {
                None => {
                    return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                        AlternativeResolveError {
                            error_type: TargetPathNotRecognized,
                        },
                    ))
                }
                Some(value) => value,
            }
                .to_string(),
        )]);
        for value in 0..((first_empty_line_pos - 2) / 2) {
            let key = (&lines)
                .get(2 + value * 2)
                .map(|value| (*value).to_string());
            let value = (&lines)
                .get(3 + value * 2)
                .map(|value| (*value).to_string());

            (&mut targets).insert(
                match key {
                    None => {
                        return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                            AlternativeResolveError {
                                error_type: TargetPathNotRecognized,
                            },
                        ))
                    }
                    Some(value) => value,
                },
                match value {
                    None => {
                        return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                            AlternativeResolveError {
                                error_type: TargetPathNotRecognized,
                            },
                        ))
                    }
                    Some(value) => value,
                },
            );
        }

        // Must be one plus length of HashMap.
        // It's because there are two or three types as below and each type must be declared in single LinkItem.
        // First: master path
        // Second: priority value
        // Third and go on: slave path (optional, can be multiple)
        // And master and slave path are managed by targets as HashMap like it is stated above
        // so it must be one plus length of targets. (Assuming the length is more than or equal to one)
        let minimum_line_length = 1 + (&targets).len();

        // Declare where to find target file paths as of
        // and it must be just next position of first_empty_line_pos.
        let target_line_start_pos = first_empty_line_pos + 1;

        // Subtract first_empty_line_pos from line length of file
        // and represent size of lines target file paths should be in
        let target_raw_length = f64::from(((&lines).len() - first_empty_line_pos - 1) as i32);

        // Predicate how many LinkItem is in data
        let iteration_count =
            (target_raw_length / f64::from(minimum_line_length as i32)).floor() as usize;

        // Register LinkItems using iteration of targets
        for value in 0..iteration_count {
            let mut link_paths: IndexSet<LinkPath> = IndexSet::default();

            for (index, (name, target_path)) in (&targets).iter().enumerate() {
                let target_path_pos = match index {
                    // Must be target_line_start_pos plus result of minimum_line_length multiplies by
                    // value indicates current index of iteration if index is zero.
                    // target_line_start_pos always means first line to read target data.
                    // Result of minimum_line_length multiplies by value means how many lines to skip
                    // in order to address first line of each data.
                    // Lastly, first element of targets is always master path data as stated above.
                    0 => target_line_start_pos + minimum_line_length * value,

                    // Must be target_line_start_pos plus 1 plus index plus result of minimum_line_length multiplies by
                    // value indicates current index of iteration if index is not zero.
                    // target_line_start_pos always means first line to read target data.
                    // The second line of each data is reserved for priority (and family)
                    // so it has to get extra value of 1.
                    // index is used to indicates which slave data is about to read.
                    // Result of minimum_line_length multiplies by value means how many lines to skip
                    // in order to address first line of each data.
                    // Lastly, first element of targets is always master path data as stated above.
                    _ => target_line_start_pos + 1 + index + minimum_line_length * value,
                };

                let execution_path = match (&lines).get(target_path_pos) {
                    None => {
                        return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                            AlternativeResolveError {
                                error_type: ExecutionPathNotRecognized,
                            },
                        ));
                    }
                    Some(value) => value.to_string(),
                };

                match !(&execution_path).eq("") {
                    true => {
                        (&mut link_paths).insert(LinkPath {
                            name: name.to_string(),
                            target_path: target_path.to_string(),
                            alternative_path: execution_path,
                        });
                    }
                    false => {
                        // return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                        //     AlternativeResolveError {
                        //         error_type: ExecutionPathNotRecognized,
                        //     },
                        // ));
                    }
                }
            }

            // Must be target_line_start_pos plus 1 plus result of minimum_line_length multiplies by
            // value indicates current index of iteration if index is not zero.
            // On Fedora (, Redhat or SUSE) based OS,
            // looks like family which means nothing but just for some tag features
            // can be appended in same line as priority locates in.
            let family_priority_pos = target_line_start_pos + 1 + minimum_line_length * value;
            let family_priority_line = match (&lines).get(family_priority_pos) {
                None => {
                    return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                        AlternativeResolveError {
                            error_type: FamilyPriorityNotRecognized,
                        },
                    ))
                }
                Some(value) => value,
            };
            let family_priority: Vec<&str> = family_priority_line.split("@").skip(1).collect();
            let _family = match (&family_priority).get(0) {
                None => "",
                Some(value) => match is_os_like("fedora".to_string()).unwrap_or(false) {
                    true => *value,
                    false => "",
                },
            };
            let family = match _family.eq("") {
                true => None,
                false => Some(_family.to_string())
            };
            let _priority = match (&family_priority).len() {
                0 => &family_priority_line,
                _ => match is_os_like("fedora".to_string()).unwrap_or(false) {
                    true => (&family_priority)
                        .get(1)
                        .map(|value| *value)
                        .unwrap_or(&family_priority_line),
                    false => &family_priority_line,
                },
            }.to_string();
            let priority = match i32::from_str(&_priority) {
                Ok(value) => value,
                Err(error) => {
                    return Err(IOParseAlternativeResolveError::ParseIntError(error));
                }
            };

            (&mut link_group).items.insert(LinkItem {
                family,
                priority,
                paths: link_paths,
            });
        }

        // Sort LinkItem by descending using priority
        (&mut link_group)
            .items
            .sort_unstable_by(|a, b| {
                match (f64::from(a.priority) - f64::from(b.priority)).floor() {
                    value if value > 0.0 => Ordering::Less,
                    value if value < 0.0 => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            });

        // Determine selected LinkItem currently
        if auto_selection {
            (&mut link_group).selected = None;
        } else {
            let converted = match i32::from_str(&raw_auto_selection) {
                Ok(value) => value,
                Err(error) => {
                    return Err(IOParseAlternativeResolveError::ParseIntError(error));
                }
            };

            (&mut link_group).selected = Some(converted as isize);
        }

        (&mut result).alternatives.insert(link_group);
    }

    Ok(result)
}

pub fn convert_alt_config_to_hashmap(
    config: &AltConfig,
) -> Result<IndexMap<String, String>, IOParseAlternativeResolveError> {
    let mut result: IndexMap<String, String> = IndexMap::new();

    for group in config.alternatives.iter() {
        let file_name = &group.filename;
        let mut file_lines: Vec<String> = Vec::new();

        // First line must be selection data
        match group.selected {
            None => {
                (&mut file_lines).push(format!("auto"));
            }
            Some(value) => {
                (&mut file_lines).push(format!("{}", value));
            }
        }

        // Second line must be path of master
        let master_path = match group.items.iter()
            .nth(0)
            .map(|value| {
                value.paths.iter()
                    .nth(0)
                    .map(|value| value.target_path.to_string())
            })
            .flatten()
        {
            None => {
                return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                    AlternativeResolveError {
                        error_type: MasterPathNotRecognized,
                    },
                ))
            }
            Some(value) => value,
        };
        (&mut file_lines).push(format!("{}", &master_path));

        // Read lines until first empty line and they must be slaves' data
        let mut data_set: IndexSet<String> = IndexSet::new();
        for item in group.items.iter() {
            for (index, path) in item.paths.iter().enumerate() {
                if index == 0 {
                    continue;
                }

                (&mut data_set).insert(format!("{}|@|{}", path.name, path.target_path));
            }
        }

        // Register these data
        for value in data_set.iter() {
            let value2: Vec<&str> = value.split("|@|").collect();

            (&mut file_lines).push(format!("{}", value2[0]));
            (&mut file_lines).push(format!("{}", value2[1]));
        }

        // Count required path length
        let data_path_length = ((&file_lines).len() - 2) / 2;

        // Append first empty line
        (&mut file_lines).push(format!(""));

        for item in group.items.iter() {
            let master_path = match item.paths.get_index(0) {
                None => {
                    return Err(IOParseAlternativeResolveError::AlternativeResolveError(
                        AlternativeResolveError {
                            error_type: MasterPathNotRecognized,
                        },
                    ));
                }
                Some(value) => value,
            };

            for index in 0..2 + data_path_length {
                if index == 0 {
                    (&mut file_lines).push(format!("{}", master_path.alternative_path));
                    continue;
                }

                if index == 1 {
                    (&mut file_lines).push(format!("{}", item.priority));
                    continue;
                }

                match item.paths.get_index(index - 1) {
                    None => {
                        (&mut file_lines).push(format!(""));
                    }
                    Some(value) => {
                        (&mut file_lines).push(format!("{}", value.alternative_path));
                    }
                };
            }
        }

        if (&file_lines).len() == 0 || !(&file_lines).last().unwrap().eq("") {
            (&mut file_lines).push(format!(""));
        }

        // EOL
        (&mut file_lines).push(format!(""));

        (&mut result).insert(file_name.to_string(), file_lines.join("\n"));
    }

    Ok(result)
}
