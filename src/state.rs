use std::error::Error;
use std::fs::{read_to_string, write};
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::fencer::Fencer;
use crate::round::Round;

#[derive(Serialize, Deserialize)]
pub struct State {
    fencers: Rc<[Fencer]>,
    rounds: Vec<Ro>,
}

#[derive(Serialize, Deserialize)]
struct Ro(#[serde(with = "serde_traitobject")] Box<dyn Round>);

impl State {
    pub fn save(&self, p: &str) -> Result<(), Box<dyn Error>> {
        write(p, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn load(p: &str) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_str::<State>(&read_to_string(p)?)?)
    }
}
