use crate::common_models::models::link_item::LinkItem;

pub trait LinkItemSearch {
    fn find_items_by_family<'t>(&'t self, family: &'t String) -> Vec<&'t LinkItem>;
    fn find_related_items_by_master_name<'t>(&'t self, name: &'t String) -> Vec<&'t LinkItem>;
}
