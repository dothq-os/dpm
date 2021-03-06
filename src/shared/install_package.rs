use std::{io::Error, path::Path};

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
        let parent_dir = Path::new(&pi.move_to).parent().unwrap().to_str().unwrap();

        commands += &format!(
            "rm -f {} && rsync --mkpath \"{}\" \"{}\" -r --delete\n",
            &pi.move_to, &pi.real, &parent_dir
        )
    });

    let (code, output, error) = run_script::run(&commands, &vec![], &options).unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);

    Ok(())
}
