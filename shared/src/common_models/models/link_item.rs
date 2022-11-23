use indexmap::IndexSet;
use crate::common_models::models::link_path::LinkPath;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkItem {
    /**
       For Fedora, Redhat and SUSE distros
    **/
    pub family: Option<String>,
    pub priority: i32,
    /**
       Contains [LinkPath].
       First item of this vector is always master path and the rest are slave path.
    **/
    pub paths: IndexSet<LinkPath>,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone, PartialEq, Eq)]
pub struct LinkItem {
    /**
       For Fedora, Redhat and SUSE distros
    **/
    pub family: Option<String>,
    pub priority: i32,
    /**
       Contains [LinkPath].
       First item of this vector is always master path and the rest are slave path.
    **/
    pub paths: IndexSet<LinkPath>,
}
