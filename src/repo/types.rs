use deb_rs::shared::PackageWithVersion;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Package {
    filename: String,
    size: u64,
    md5sum: String,
    sha1: String,
    sha256: String,
    description: String,
    install_size: u64,
    // Depends et al: <https://www.debian.org/doc/debian-policy/ch-relationships.html#s-binarydeps>
    pub depends: Vec<PackageWithVersion>,
    pub pre_depends: Vec<PackageWithVersion>,
    pub recommends: Vec<PackageWithVersion>,
    pub suggests: Vec<PackageWithVersion>,
    pub enhances: Vec<PackageWithVersion>,
    pub breaks: Vec<PackageWithVersion>,
    pub conflicts: Vec<PackageWithVersion>,
}
