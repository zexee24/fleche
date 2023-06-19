use std::rc::Rc;

use crate::fencer::{Bout, BoutScore, Fencer};

pub trait Round {
    fn get_fencers(&self) -> &[Rc<Fencer>];
    fn add_results(&mut self, b: BoutScore);
    fn is_done(&self) -> bool;
    fn get_bouts(&self) -> Vec<Bout>;
}

pub struct Poule {
    fencers: Vec<Rc<Fencer>>,
    bouts: Vec<Bout>,
}

impl Round for Poule {
    fn get_fencers(&self) -> &[Rc<Fencer>] {
        &self.fencers
    }

    fn add_results(&mut self, bout: BoutScore) {
        if let Some(n) = self.bouts.iter().position(|b| match b {
            Bout::Upcoming { left, right } => bout.has_fencers(left, right),
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
}

#[cfg(test)]
mod tests {
    use crate::fencer::Fencer;
    use crate::variants::Round;

    use super::Poule;

    const MOCK_SIZE: i64 = 10_000;

    #[test]
    fn test_poule_has_all_bouts() {
        let mut l: Vec<Fencer> = Vec::new();

        for i in 0..MOCK_SIZE {
            l.push(Fencer::new(
                i.to_string(),
                i.to_string(),
                "TEST".to_string(),
            ))
        }

        let p = Poule::new(l);
        assert_eq!(p.get_bouts().len() as i64, MOCK_SIZE * (MOCK_SIZE - 1) / 2)
    }
}
