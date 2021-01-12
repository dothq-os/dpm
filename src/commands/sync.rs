use std::io::Error;

use crate::repo::*;

pub async fn sync() -> Result<(), Box<dyn std::error::Error>> {
    PackageCache::init().await?;
    Ok(())
}
