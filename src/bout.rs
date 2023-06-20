use std::rc::Rc;

use serde::{Serialize, Deserialize};

use crate::fencer::Fencer;

//🧹This is kinda a terrible way to implement this🧹
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bout {
    Upcoming { left: Rc<Fencer>, right: Rc<Fencer> },
    Finished(BoutScore),
}

impl Bout{
    pub fn has_fencers(&self, f: &Fencer, s: &Fencer) -> bool {
        self.contains_fencer(f) && self.contains_fencer(s)
    }

    pub fn contains_fencer(&self, f: &Fencer) -> bool{
        match self{
            Bout::Upcoming { left, right } => &**left == f || &**right == f,
            Bout::Finished(b) => &*b.winner == f || &*b.loser == f,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoutScore {
    winner: Rc<Fencer>,
    winner_score: i32,
    loser: Rc<Fencer>,
    loser_score: i32,
}

impl BoutScore {
    pub fn new(winner: Rc<Fencer>, winner_score: i32, loser: Rc<Fencer>, loser_score: i32) -> Self {
        BoutScore {
            winner,
            winner_score,
            loser,
            loser_score,
        }
    }
}
