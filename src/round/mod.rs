use std::rc::Rc;

pub mod poule;

use crate::bout::{Bout, BoutScore};
use crate::fencer::Fencer;

pub enum RoundTypes{
    Poule,
    Cup,
}

pub trait Round: serde_traitobject::Serialize + serde_traitobject::Deserialize{
    fn get_fencers(&self) -> &[Rc<Fencer>];
    fn add_results(&mut self, b: BoutScore);
    fn is_done(&self) -> bool;
    fn get_bouts(&self) -> Vec<Bout>;
}

