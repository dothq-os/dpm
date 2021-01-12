use std::io::Error;

use crate::repo::*;

pub async fn sync() -> Result<(), Error> {
    PackageCache::init().await?;
    Ok(())
}
