use std::error::Error;
use std::fs::{read_to_string, write};

use crate::state::State;

pub fn save(p: &str, s: State) -> Result<(), Box<dyn Error>> {
    write(p, serde_json::to_string(&s)?)?;
    Ok(())
}

pub fn load(p: &str) -> Result<State, Box<dyn Error>> {
    Ok(serde_json::from_str::<State>(&read_to_string(p)?)?)
}
