#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkPath {
    pub name: String,
    pub target_path: String,
    pub alternative_path: String,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LinkPath {
    pub name: String,
    pub target_path: String,
    pub alternative_path: String,
}
