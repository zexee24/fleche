use std::rc::Rc;

use serde::{Serialize, Deserialize};

use crate::fencer::Fencer;
use crate::round::Round;

#[derive(Serialize, Deserialize)]
pub struct State{
    fencers: Rc<[Fencer]>,
    rounds: Vec<Ro>
}

#[derive(Serialize, Deserialize)]
struct Ro(
    #[serde(with = "serde_traitobject")]
    Box<dyn Round>
);
