#[derive(Clone, PartialEq, Debug)]
pub struct Fencer {
    firstname: String,
    surname: String,
    nationality: String,
}

#[derive(Debug, Clone)]
pub enum Bout<'a> {
    Upcoming { left: &'a Fencer, right: &'a Fencer },
    Finished(BoutScore<'a>),
}

#[derive(Debug, Clone)]
pub struct BoutScore<'a> {
    winner: &'a Fencer,
    winner_score: i32,
    loser: &'a Fencer,
    loser_score: i32,
}

impl <'a> BoutScore<'a> {
    pub fn new(winner: &Fencer, winner_score: i32, loser: &Fencer, loser_score: i32) -> Self {
        BoutScore {
            winner,
            winner_score,
            loser,
            loser_score,
        }
    }

    pub fn has_fencers(&self, f: &Fencer, s: &Fencer) -> bool {
        (self.winner == f && self.loser == s) || (self.loser == f && self.winner == s)
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
