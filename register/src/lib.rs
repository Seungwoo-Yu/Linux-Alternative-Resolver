pub mod impls;
pub mod traits;

#[cfg(target_os = "linux")]
const ETC_ALTERNATIVE_PATH_STRING: &str = "/etc/alternatives";
