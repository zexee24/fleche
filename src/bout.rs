use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::fencer::Fencer;

//🧹This is kinda a terrible way to implement this🧹
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bout {
    Upcoming { left: Rc<Fencer>, right: Rc<Fencer> },
    Finished(BoutScore),
}

impl Bout {
    pub fn has_fencers(&self, f: &Fencer, s: &Fencer) -> bool {
        self.contains_fencer(f) && self.contains_fencer(s)
    }

    pub fn contains_fencer(&self, f: &Fencer) -> bool {
        match self {
            Bout::Upcoming { left, right } => &**left == f || &**right == f,
            Bout::Finished(b) => &*b.winner == f || &*b.loser == f,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoutScore {
    pub winner: Rc<Fencer>,
    pub winner_score: u16,
    pub loser: Rc<Fencer>,
    pub loser_score: u16,
}

impl BoutScore {
    pub fn new(winner: Rc<Fencer>, winner_score: u16, loser: Rc<Fencer>, loser_score: u16) -> Self {
        BoutScore {
            winner,
            winner_score,
            loser,
            loser_score,
        }
    }

    pub fn did_win(&self, fencer: &Fencer) -> Option<bool> {
        if &*self.winner == fencer {
            return Some(true);
        }

        if &*self.loser == fencer {
            return Some(false);
        }
        None
    }

    pub fn get_score(&self, fencer: &Fencer) -> Option<u16> {
        if &*self.winner == fencer {
            return Some(self.winner_score);
        }

        if &*self.loser == fencer {
            return Some(self.loser_score);
        }
        None
    }
    pub fn get_for_opponent(&self, fencer: &Fencer) -> Option<u16> {
        match &*self.winner == fencer {
            true => self.get_score(&self.loser),
            false => self.get_score(&self.winner),
        }
    }
}
