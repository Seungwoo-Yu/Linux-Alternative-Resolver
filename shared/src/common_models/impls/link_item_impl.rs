use std::hash::{Hash, Hasher};
use crate::common_models::models::link_item::LinkItem;
use crate::common_models::models::link_path::LinkPath;

impl Hash for LinkItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.priority.hash(state);
        self.family.hash(state);
        let paths: Vec<&LinkPath> = self.paths.iter().collect();
        paths.hash(state);
    }
}