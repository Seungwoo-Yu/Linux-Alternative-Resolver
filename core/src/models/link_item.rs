use crate::models::link_path::LinkPath;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Clone, Serialize, Deserialize)]
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
    pub paths: Vec<LinkPath>,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone)]
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
    pub paths: Vec<LinkPath>,
}
