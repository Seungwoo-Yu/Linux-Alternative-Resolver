use crate::models::link_group::LinkGroup;

pub trait LinkGroupSearch {
    fn find_group_by_name<'t>(&'t self, name: &'t String) -> Option<&'t LinkGroup>;
}
