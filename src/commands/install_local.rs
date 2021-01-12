use std::io::Error;

use byte_unit::Byte;
use deb_rs::file::Deb;

extern crate question;
use question::{Answer, Question};

use crate::shared::install;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn install_local(path: String) -> Result<(), Error> {
    let mut deb = Deb::new(string_to_static_str(path));
    println!("Extracting file");
    deb.extract()?;

    let control = deb.retrieve_control()?;

    println!("Note: Dependencies are currently ignored");
    println!(
        "Install size: {}",
        Byte::from_bytes(control.install_size.unwrap() as u128)
            .get_appropriate_unit(false)
            .to_string()
    );

    let answer = Question::new("Do you want to install?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    if answer == Answer::YES {
        install(&deb)?;
    }

    Ok(())
}
