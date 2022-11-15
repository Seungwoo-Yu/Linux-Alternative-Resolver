use crate::models::link_item::LinkItem;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Clone, Serialize, Deserialize)]
pub struct LinkGroup {
    pub name: String,
    pub selected: Option<isize>,
    pub items: Vec<LinkItem>,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone)]
pub struct LinkGroup {
    pub name: String,
    pub selected: Option<isize>,
    pub items: Vec<LinkItem>,
}
