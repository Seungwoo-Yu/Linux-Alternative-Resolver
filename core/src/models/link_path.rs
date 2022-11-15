#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Clone, Serialize, Deserialize)]
pub struct LinkPath {
    pub name: String,
    pub target_path: String,
    pub alternative_path: String,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone)]
pub struct LinkPath {
    pub name: String,
    pub target_path: String,
    pub alternative_path: String,
}
