use crate::repo::*;

pub async fn sync() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initialize");
    let mut package_cache = PackageCache::with_datastore(false).await?;
    println!("Sync");
    package_cache.data.sync_package_cache().await?;
    println!("Updating");
    package_cache.update()?;

    Ok(())
}
