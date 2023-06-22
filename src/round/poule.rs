use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::bout::{Bout, BoutScore};
use crate::fencer::Fencer;

use super::Round;

#[derive(Serialize, Deserialize)]
pub struct Poule {
    fencers: Vec<Rc<Fencer>>,
    bouts: Vec<Bout>,
}

struct Results<'a> {
    matches: &'a [Bout],
    wins: i32,
    given: i32,
    recieved: i32,
}

impl Round for Poule {
    fn get_fencers(&self) -> &[Rc<Fencer>] {
        &self.fencers
    }

    fn add_results(&mut self, bout: BoutScore) {
        if let Some(n) = self.bouts.iter().position(|b| match b {
            Bout::Upcoming { left, right } => b.has_fencers(left, right),
            Bout::Finished(_) => false,
        }) {
            self.bouts.push(Bout::Finished(bout));
            self.bouts.swap_remove(n);
        }
    }
    fn is_done(&self) -> bool {
        let f = self.fencers.len();
        self.bouts.len() == f * (f - 1) / 2
    }

    fn get_bouts(&self) -> Vec<Bout> {
        self.bouts.clone()
    }
}

impl Poule {
    fn new(fencers: Vec<Fencer>) -> Self {
        let mut bouts = Vec::new();
        let fencers: Vec<Rc<Fencer>> = fencers.iter().map(|x| Rc::new(x.clone())).collect();
        let mut f = fencers.iter();
        while let Some(first) = f.next() {
            let iterable = f.clone();
            for second in iterable {
                bouts.push(Bout::Upcoming {
                    left: first.clone(),
                    right: second.clone(),
                })
            }
        }
        Poule { fencers, bouts }
    }

    fn solve<'a>(&self) -> Vec<(Rc<Fencer>, Results<'a>)> {
        self.fencers
            .iter()
            .map(|x| (x.clone(), self.get_results_for_fencer(&x)))
            .collect()
    }

    fn get_results_for_fencer<'a>(&self, fencer: &Fencer) -> Results<'a> {
        self.bouts.iter().filter(|x| x.contains_fencer(fencer));
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::fencer::Fencer;
    use crate::round::Round;

    use super::Poule;

    const MOCK_SIZE: i64 = 10_000;

    #[test]
    fn test_poule_has_all_bouts() {
        let mut l: Vec<Fencer> = Vec::new();

        for i in 0..MOCK_SIZE {
            l.push(Fencer::new(i.to_string(), None, None))
        }

        let p = Poule::new(l);
        assert_eq!(p.get_bouts().len() as i64, MOCK_SIZE * (MOCK_SIZE - 1) / 2)
    }
}
