use crate::models::alt_config::AltConfig;
use crate::models::link_group::LinkGroup;
use crate::models::link_item::LinkItem;
use crate::traits::link_group_search::LinkGroupSearch;
use crate::traits::link_item_search::LinkItemSearch;

impl LinkGroupSearch for AltConfig {
    fn find_group_by_name<'t>(&'t self, name: &'t String) -> Option<&'t LinkGroup> {
        for value in self.alternatives.iter() {
            if value.name.eq(name) {
                return Some(value);
            }
        }

        None
    }
}

impl LinkItemSearch for AltConfig {
    fn find_items_by_family<'t>(&'t self, family: &'t String) -> Vec<&'t LinkItem> {
        let mut result: Vec<&'t LinkItem> = vec![];

        for value in self.alternatives.iter() {
            for value2 in value.items.iter() {
                match &value2.family {
                    None => {}
                    Some(value3) => {
                        if value3.eq(family) {
                            (&mut result).push(value2);
                        }
                    }
                }
            }
        }

        result
    }

    fn find_related_items_by_master_name<'t>(&'t self, name: &'t String) -> Vec<&'t LinkItem> {
        let mut result: Vec<&'t LinkItem> = vec![];

        for value in self.alternatives.iter() {
            if value.name.eq(name) {
                for value2 in value.items.iter() {
                    (&mut result).push(value2);
                }
                break;
            }
        }

        result
    }
}
