use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub struct Fencer {
    firstname: String,
    surname: String,
    nationality: String,
}

#[derive(Debug, Clone)]
pub enum Bout {
    Upcoming { left: Rc<Fencer>, right: Rc<Fencer>},
    Finished(BoutScore),
}

#[derive(Debug, Clone)]
pub struct BoutScore {
    winner: Rc<Fencer>,
    winner_score: i32,
    loser: Rc<Fencer>,
    loser_score: i32,
}

impl BoutScore{
    pub fn new(winner: Rc<Fencer>, winner_score: i32, loser: Rc<Fencer>, loser_score: i32) -> Self {
        BoutScore {
            winner,
            winner_score,
            loser,
            loser_score,
        }
    }

    pub fn has_fencers(&self, f: &Fencer, s: &Fencer) -> bool {
        (&*self.winner == f && &*self.loser == s) || (&*self.loser == f && &*self.winner == s)
    }
}

impl Fencer {
    pub fn new(name: String, surname: String, nationality: String) -> Self {
        Fencer {
            firstname: name,
            surname,
            nationality,
        }
    }
}
