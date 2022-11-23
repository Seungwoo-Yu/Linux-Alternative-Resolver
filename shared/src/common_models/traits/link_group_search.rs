use crate::common_models::models::link_group::LinkGroup;

pub trait LinkGroupSearch {
    fn find_group_by_name<'t>(&'t self, name: &'t String) -> Option<&'t LinkGroup>;
}
