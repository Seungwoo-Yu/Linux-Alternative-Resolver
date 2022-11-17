use indexmap::IndexSet;
use crate::common_models::models::link_item::LinkItem;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkGroup {
    pub name: String,
    pub selected: Option<isize>,
    pub items: IndexSet<LinkItem>,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone, PartialEq, Eq)]
pub struct LinkGroup {
    pub name: String,
    pub selected: Option<isize>,
    pub items: IndexSet<LinkItem>,
}
