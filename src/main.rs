#[macro_use]
extern crate clap;
use clap::App;

mod commands;
mod repo;
mod shared;

use commands::{install_local, sync};

#[tokio::main]
pub async fn main() {
    let yaml = load_yaml!("cli.yaml");
    let mut app = App::from_yaml(yaml);
    let matches = app.clone().get_matches();

    match matches.subcommand_name() {
        Some("local-install") => {
            let path = matches.value_of("PATH").unwrap().to_string();
            install_local(path).unwrap();
        }

        Some("search") => {
            let _query = matches.value_of("QUERY").unwrap();
        }

        Some("sync") => sync().await.unwrap(),

        Some(_) => {
            println!("Unknown command. Use --help for a list fo commands");
        }

        None => app.print_help().unwrap(),
    }

    if let Some(matches) = matches.subcommand_matches("local-install") {
        if matches.is_present("PATH") {
            let path = matches.value_of("PATH").unwrap().to_string();
            install_local(path).unwrap();
        }
    }
}
