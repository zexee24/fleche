use std::error::Error;
use std::fs::{read_to_string, write};
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::fencer::Fencer;
use crate::round::group::Group;
use crate::round::Round;

#[derive(Serialize, Deserialize)]
pub struct State {
    fencers: Rc<[Rc<Fencer>]>,
    rounds: Vec<Group>,
}

impl State {
    pub fn save(&self, p: &str) -> Result<(), Box<dyn Error>> {
        write(p, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn load(p: &str) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_str::<State>(&read_to_string(p)?)?)
    }

    pub fn fencer_amout(&self) -> usize {
        self.fencers.len()
    }

    pub fn last_round(&self) -> Option<&Group> {
        self.rounds.last()
    }

    pub fn add_round(&mut self, r: Group) {
        self.rounds.push(r);
    }
    pub fn get_fencers(&self) -> Rc<[Rc<Fencer>]> {
        self.fencers.clone()
    }
}
