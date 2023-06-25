use std::fmt::Display;
use std::rc::Rc;

pub mod poule;

use strum::EnumIter;

use crate::bout::{Bout, BoutScore};
use crate::fencer::Fencer;

#[derive(EnumIter)]
pub enum RoundTypes {
    Poule,
    Cup,
}

impl Display for RoundTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            RoundTypes::Poule => "Poule",
            RoundTypes::Cup => "Cup",
        };
        write!(f, "{}", w)
    }
}

pub trait Round: serde_traitobject::Serialize + serde_traitobject::Deserialize {
    fn get_fencers(&self) -> &[Rc<Fencer>];
    fn add_results(&mut self, b: BoutScore);
    fn is_done(&self) -> bool;
    fn get_bouts(&self) -> Vec<Rc<Bout>>;
}
