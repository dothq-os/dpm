use deb_rs::shared::PackageWithVersion;
use serde::{Deserialize, Serialize};

use crate::shared::{deb_control::Paragraph, paragraph_contains};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Package {
    filename: String,
    size: u64,
    md5sum: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    description: Option<String>,
    install_size: Option<u64>,
    // Depends et al: <https://www.debian.org/doc/debian-policy/ch-relationships.html#s-binarydeps>
    pub depends: Vec<PackageWithVersion>,
    pub pre_depends: Vec<PackageWithVersion>,
    pub recommends: Vec<PackageWithVersion>,
    pub suggests: Vec<PackageWithVersion>,
    pub enhances: Vec<PackageWithVersion>,
    pub breaks: Vec<PackageWithVersion>,
    pub conflicts: Vec<PackageWithVersion>,
}

impl Package {
    pub fn from_paragraph(par: Paragraph) -> Result<Self, Box<dyn std::error::Error>> {
        let filename = Package::get_control_string(&par, "Filename");
        let size = Package::get_control_string(&par, "Size").parse()?;

        let md5sum = Package::get_control_option_str(&par, "MD5Sum");
        let sha1 = Package::get_control_option_str(&par, "SHA1");
        let sha256 = Package::get_control_option_str(&par, "SHA256");
        let description = Package::get_control_option_str(&par, "Description-md5");
        let install_size = Package::get_control_option_str(&par, "Installed-Size");

        let install_size = Package::str_option_to_number(install_size);

        // Depends et al
        let depends = Package::get_package_name(&par, "Depends");
        let pre_depends = Package::get_package_name(&par, "Pre-Depends");
        let recommends = Package::get_package_name(&par, "Recommends");
        let suggests = Package::get_package_name(&par, "Suggests");
        let enhances = Package::get_package_name(&par, "Enhances");
        let breaks = Package::get_package_name(&par, "Breaks");
        let conflicts = Package::get_package_name(&par, "Conflicts");

        Ok(Package {
            filename,
            size,
            md5sum,
            sha1,
            sha256,
            description,
            install_size,
            depends,
            pre_depends,
            recommends,
            suggests,
            enhances,
            breaks,
            conflicts,
        })
    }

    fn get_control_string(control: &Paragraph, query: &str) -> String {
        paragraph_contains(control.clone(), query.to_string())
            .unwrap()
            .value
    }

    fn str_option_to_number(option: Option<String>) -> Option<u64> {
        if let Some(option) = option {
            Some(option.parse().unwrap())
        } else {
            None
        }
    }

    fn get_control_option_str(control: &Paragraph, query: &str) -> Option<String> {
        let item = paragraph_contains(control.clone(), query.to_string());

        if let Some(item) = item {
            Some(item.value)
        } else {
            None
        }
    }

    fn get_package_name(control: &Paragraph, query: &str) -> Vec<PackageWithVersion> {
        let item = paragraph_contains(control.clone(), query.to_string());

        let mut deps = Vec::new();

        if let Some(item) = item {
            let input: Vec<&str> = item.value.split(',').collect();

            input
                .into_iter()
                .for_each(|dep| deps.push(PackageWithVersion::from_str(dep)));
        }

        deps
    }
}
