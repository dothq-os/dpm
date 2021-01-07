use std::{fs, io::Error};

use deb_rs::file::Deb;

pub fn install(file: &Deb) -> Result<(), Error> {
    let install_paths = file.install_tree()?;

    if sudo::check() != sudo::RunningAs::Root {
        panic!("Must be run as root")
    }

    install_paths.into_iter().for_each(|pi| {
        fs::copy(pi.real, pi.move_to).unwrap();
        ()
    });

    Ok(())
}
