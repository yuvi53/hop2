use clap::{Arg, ArgAction, Command};
use hop2::{
    add_path,
    data::{load, save},
    find_matches, set_defaults,
};
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("whatever")
        .arg(Arg::new("add").long("add").action(ArgAction::Set))
        .arg(Arg::new("dir").long("dir").action(ArgAction::Set))
        .get_matches();
    if let Some(dir) = matches.get_one::<String>("dir") {
        println!("{}", find_matches(&dir, load(set_defaults())).display());
    }
    if let Some(path) = matches.get_one::<String>("add") {
        let mut entries = load(set_defaults());
        add_path(PathBuf::from(&path), &mut entries, None);
        save(entries)?;
    }
    Ok(())
}
