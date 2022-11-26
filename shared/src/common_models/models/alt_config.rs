use indexmap::IndexSet;
use crate::common_models::models::link_group::LinkGroup;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Clone, Serialize, Deserialize)]
pub struct AltConfig {
    pub alternatives: IndexSet<LinkGroup>,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone)]
pub struct AltConfig {
    pub alternatives: IndexSet<LinkGroup>,
}
