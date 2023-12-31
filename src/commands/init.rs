use std::error::Error;
use std::ffi::OsString;

use inquire::Editor;

use crate::fencer::Fencer;

pub fn init() -> Result<(), Box<dyn Error>> {
    let fencer_list = get_fencers_from_user()?;

    println!("Entered {:?} fencers", fencer_list.len(),);

    Ok(())
}

fn get_fencers_from_user() -> Result<Vec<Fencer>, Box<dyn Error>> {
    let sys_editor = std::env::var("EDITOR")?;
    let res = Editor::new("Enter fencers, First name, Surname and Nationality on diffirent lines. With blank lines between fencers")
        .with_editor_command(&OsString::from(sys_editor))
        .prompt()?;

    Ok(res.split("\n\n").filter_map(get_next_fencer).collect())
}

fn get_next_fencer(lines: &str) -> Option<Fencer> {
    let mut l = lines.lines();
    let first_name = l.next()?;
    let surname = l.next();
    let nat = l.next();

    Some(Fencer::new(
        first_name.to_string(),
        surname.map(|s| s.to_string()),
        nat.map(|s| s.to_string()),
    ))
}
