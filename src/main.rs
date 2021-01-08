#[macro_use]
extern crate clap;
use clap::App;

mod install_local;
mod shared;

use install_local::install_local;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("local-install") {
        if matches.is_present("PATH") {
            let path = matches.value_of("PATH").unwrap().to_string();
            install_local(path).unwrap();
        }
    }
}
