use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::io::Error;

use crate::shared::DataStore;

const DEBIAN_VERSION: &str = "experimental";
const MIRROR: &str = "https://mirror.aarnet.edu.au/pub/debian/";
const ARCH: &str = "amd64";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct Package {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PackageCache {
    time_stamp: DateTime<Utc>,
    main: Vec<Package>,
    contrib: Vec<Package>,
    non_free: Vec<Package>,
}

impl PackageCache {
    pub async fn init() -> Result<Self, Error> {
        let cache_time = Duration::hours(-1);

        let default = PackageCache {
            time_stamp: DateTime::from_utc(NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 1, 1), Utc), // A time stamp that will always be outside the current time
            main: vec![],
            contrib: vec![],
            non_free: vec![],
        };

        let package_cache = DataStore::new("repo_cache", &default)?;

        let diff = {
            let now = Utc::now();
            package_cache.data.time_stamp.signed_duration_since(now)
        };

        if diff < cache_time {
            // Generate package cache
            package_cache.data.sync_packages().await?;
        }

        Ok(package_cache.data)
    }

    /**
     * Syncs the package cache with the debian package cache
     */
    pub async fn sync_packages(&self) -> Result<(), Error> {
        // Shared client to improve performance
        let client = reqwest::Client::new();

        let main = client
            .get(&format!(
                "{}dists/{}/main/binary-{}/Packages.xz",
                MIRROR, DEBIAN_VERSION, ARCH
            ))
            .await?
            .bytes()
            .await?;

        Ok(())
    }
}
