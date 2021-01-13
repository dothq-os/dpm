use chrono::{DateTime, Duration, NaiveDate, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::Read;
use xz2::read::XzDecoder;

mod types;

use crate::shared::{deb_control::control_from_string, DataStore};
use types::Package;

const DEBIAN_VERSION: &str = "experimental";
const MIRROR: &str = "https://mirror.aarnet.edu.au/pub/debian/";
const ARCH: &str = "amd64";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PackageCache {
    time_stamp: DateTime<Utc>,
    main: Vec<Package>,
    contrib: Vec<Package>,
    non_free: Vec<Package>,
}

impl PackageCache {
    pub async fn with_datastore(
        should_sync: bool,
    ) -> Result<DataStore<Self>, Box<dyn std::error::Error>> {
        let cache_time = Duration::hours(-1);

        let default = PackageCache {
            time_stamp: DateTime::from_utc(NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 1, 1), Utc), // A time stamp that will always be outside the current time
            main: vec![],
            contrib: vec![],
            non_free: vec![],
        };

        let mut package_cache_ds = DataStore::new("repo_cache", &default)?;
        let package_cache = &mut package_cache_ds.data;

        let diff = {
            let now = Utc::now();
            package_cache.time_stamp.signed_duration_since(now)
        };

        if should_sync && diff < cache_time {
            // Generate package cache
            package_cache.sync_package_cache().await?;
        }

        Ok(package_cache_ds)
    }

    pub async fn without_datastore(should_sync: bool) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(PackageCache::with_datastore(should_sync).await?.data)
    }

    async fn download_bytes(
        client: &Client,
        path: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(client.get(path).send().await?.bytes().await?.to_vec())
    }

    pub async fn sync_package_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (main, contrib, non_free) = self.download_packages().await.unwrap();
        self.parse_controls(main, contrib, non_free)?;

        Ok(())
    }

    /**
     * Syncs the package cache with the debian package cache
     */
    pub async fn download_packages(
        &self,
    ) -> Result<(String, String, String), Box<dyn std::error::Error>> {
        //* Download package lists

        // Shared client to improve performance
        let client = reqwest::Client::new();
        let mut decompressor;

        let main = &*PackageCache::download_bytes(
            &client,
            &format!(
                "{}dists/{}/main/binary-{}/Packages.xz",
                MIRROR, DEBIAN_VERSION, ARCH
            ),
        )
        .await?;

        decompressor = XzDecoder::new(main);
        let mut main = String::new();
        decompressor.read_to_string(&mut main)?;

        let contrib = &*PackageCache::download_bytes(
            &client,
            &format!(
                "{}dists/{}/contrib/binary-{}/Packages.xz",
                MIRROR, DEBIAN_VERSION, ARCH
            ),
        )
        .await?;

        decompressor = XzDecoder::new(contrib);
        let mut contrib = String::new();
        decompressor.read_to_string(&mut contrib)?;

        let non_free = &*PackageCache::download_bytes(
            &client,
            &format!(
                "{}dists/{}/non-free/binary-{}/Packages.xz",
                MIRROR, DEBIAN_VERSION, ARCH
            ),
        )
        .await?;

        decompressor = XzDecoder::new(non_free);
        let mut non_free = String::new();
        decompressor.read_to_string(&mut non_free)?;

        Ok((main, contrib, non_free))
    }

    pub fn parse_controls(
        &mut self,
        main: String,
        contrib: String,
        non_free: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let main_control = control_from_string(main)?;
        let contrib_control = control_from_string(contrib)?;
        let non_free_control = control_from_string(non_free)?;

        self.main = main_control
            .into_iter()
            .map(|par| Package::from_paragraph(par).unwrap())
            .collect();

        self.contrib = contrib_control
            .into_iter()
            .map(|par| Package::from_paragraph(par).unwrap())
            .collect();

        self.non_free = non_free_control
            .into_iter()
            .map(|par| Package::from_paragraph(par).unwrap())
            .collect();

        Ok(())
    }
}
