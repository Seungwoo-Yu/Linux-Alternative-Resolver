use std::hash::{Hash, Hasher};
use crate::common_models::models::link_group::LinkGroup;
use crate::common_models::models::link_item::LinkItem;

impl Hash for LinkGroup {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.selected.hash(state);
        let items: Vec<&LinkItem> = self.items.iter().collect();
        items.hash(state);
    }
}