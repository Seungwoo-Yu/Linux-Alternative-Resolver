use std::{fs, io};

pub mod alternative_resolver;
pub mod impls;
pub mod traits;

pub(in crate) const POSSIBLE_PATHS: [&str; 4] = [
    // For Ubuntu distro
    // Refer https://manpages.ubuntu.com/manpages/trusty/man8/update-alternatives.8.html
    "/var/lib/dpkg/alternatives",
    // For Suse and other possible Redhat distros
    // Refer https://documentation.suse.com/es-es/sles/15-SP1/html/SLES-all/cha-update-alternative.html
    "/var/lib/rpm/alternatives",
    // Rocky Linux and other possible Redhat distros
    // Refer https://linux.die.net/man/8/update-alternatives
    "/var/lib/alternatives",
    // For rest of possible linux distros
    // Refer https://man7.org/linux/man-pages/man1/update-alternatives.1.html
    "/usr/local/var/lib/dpkg/alternatives",
];

pub(in crate) fn is_os_like(name: String) -> Result<bool, io::Error> {
    let lowercase_name = (&name).to_lowercase();
    let data = fs::read_to_string("/etc/os-release")?;

    for value in data.lines() {
        let name_condition = value.find("NAME=")
            .map(| value | value == 0)
            .and_then(| value | value.then_some(()));
        match name_condition {
            None => {}
            Some(_) => {
                let matched_name = value
                    .replace("NAME=", "")
                    .replace("\"", "")
                    .to_lowercase();

                if (&matched_name).eq(&lowercase_name) {
                    return Ok(true);
                }
            }
        }

        match value.find("ID=") {
            None => {}
            Some(_) => {
                let _matched_ids = value
                    .replace("ID=", "")
                    .replace("\"", "")
                    .to_lowercase();
                let matched_ids: Vec<&str> = (&_matched_ids).split_whitespace().collect();

                if matched_ids.contains(&lowercase_name.as_str()) {
                    return Ok(true);
                }
            }
        }

        match value.find("ID_LIKE=") {
            None => {}
            Some(_) => {
                let _matched_ids = value
                    .replace("ID_LIKE=", "")
                    .replace("\"", "")
                    .to_lowercase();
                let matched_ids: Vec<&str> = (&_matched_ids).split_whitespace().collect();

                if matched_ids.contains(&lowercase_name.as_str()) {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}
