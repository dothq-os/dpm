use std::io::Error;

use run_script::ScriptOptions;

use deb_rs::file::Deb;

pub fn install(file: &Deb) -> Result<(), Error> {
    let install_paths = file.install_tree()?;

    if sudo::check() != sudo::RunningAs::Root {
        panic!("Must be run as root")
    }

    let mut commands = String::new();
    let options = ScriptOptions::new();

    install_paths.into_iter().for_each(|pi| {
        commands += &format!(
            "mkdir -p \"{}\" && cp \"{}\" \"{}\"\n",
            &pi.move_to, &pi.real, &pi.move_to
        )
    });

    let _ = run_script::run(&commands, &vec![], &options).unwrap();

    Ok(())
}
