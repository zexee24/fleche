use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::bout::{Bout, BoutScore};
use crate::fencer::Fencer;

use super::Round;

#[derive(Serialize, Deserialize, Clone)]
pub struct Poule {
    fencers: Vec<Rc<Fencer>>,
    bouts: Vec<Rc<Bout>>,
}

#[derive(Debug)]
struct Results {
    matches: Vec<Rc<Bout>>,
    wins: u16,
    given: u16,
    recieved: u16,
}

impl Results {
    pub fn index(&self) -> i32 {
        self.given as i32 - self.recieved as i32
    }

    pub fn win_rate(&self) -> f32 {
        self.wins as f32 / self.matches.len() as f32
    }
}

impl Round for Poule {
    fn get_fencers(&self) -> &[Rc<Fencer>] {
        &self.fencers
    }

    fn add_results(&mut self, bout: BoutScore) {
        if let Some(n) = self.bouts.iter().position(|b| match &**b {
            Bout::Upcoming { left, right } => b.has_fencers(left, right),
            Bout::Finished(_) => false,
        }) {
            self.bouts.push(Rc::new(Bout::Finished(bout)));
            self.bouts.swap_remove(n);
        }
    }
    fn is_done(&self) -> bool {
        self.bouts.iter().all(|x| match &**x {
            Bout::Upcoming { left: _, right: _ } => false,
            Bout::Finished(_) => true,
        })
    }

    fn get_bouts(&self) -> Vec<Rc<Bout>> {
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
                bouts.push(Rc::new(Bout::Upcoming {
                    left: first.clone(),
                    right: second.clone(),
                }))
            }
        }
        Poule { fencers, bouts }
    }

    fn solve(&self) -> Vec<(Rc<Fencer>, Results)> {
        self.fencers
            .iter()
            .map(|x| (x.clone(), self.get_results_for_fencer(x)))
            .collect()
    }

    fn get_results_for_fencer(&self, fencer: &Fencer) -> Results {
        let relevant = self
            .bouts
            .clone()
            .into_iter()
            .filter(|x| x.contains_fencer(fencer));

        let (wins, given, recieved) = relevant.clone().fold((0, 0, 0), |acc, x| match &*x {
            Bout::Upcoming { left: _, right: _ } => acc,
            Bout::Finished(b) => (
                acc.0 + b.did_win(fencer).map_or(0, |_| 1),
                acc.1 + b.get_score(fencer).unwrap_or(0),
                acc.2 + b.get_for_opponent(fencer).unwrap_or(0),
            ),
        });

        Results {
            matches: relevant.collect(),
            wins,
            given,
            recieved,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bout::BoutScore;
    use crate::fencer::Fencer;
    use crate::round::Round;

    use super::Poule;

    const MOCK_SIZE: u32 = 1_000;

    #[test]
    fn test_poule_has_all_bouts() {
        let p = Poule::new(mock_few_fencers(MOCK_SIZE));
        assert_eq!(p.get_bouts().len() as u32, MOCK_SIZE * (MOCK_SIZE - 1) / 2)
    }

    fn mock_few_fencers(n: u32) -> Vec<Fencer> {
        let mut v = Vec::new();
        for i in 0..n {
            v.push(Fencer::new(i.to_string(), None, None))
        }

        v
    }

    #[test]
    fn test_can_solve_poule_correctly() {
        let mut p = Poule::new(mock_few_fencers(4));
        let c = p.clone();
        let f = c.get_fencers();
        /*     1 2 3 4   V Ts Tr I
         *   -------------
         * 1 | x 5 5 4 | 2 14 8  +6
         * 2 | 1 x 5 2 | 1 8  14 -6
         * 3 | 2 4 x 4 | 0 10 15 -5
         * 4 | 5 5 5 x | 3 15 10 +5
         */
        vec![
            BoutScore::new(f[0].clone(), 5, f[1].clone(), 1),
            BoutScore::new(f[0].clone(), 5, f[2].clone(), 2),
            BoutScore::new(f[3].clone(), 5, f[0].clone(), 4),
            BoutScore::new(f[1].clone(), 5, f[2].clone(), 4),
            BoutScore::new(f[3].clone(), 5, f[1].clone(), 2),
            BoutScore::new(f[3].clone(), 5, f[2].clone(), 4),
        ]
        .into_iter()
        .for_each(|x| p.add_results(x));
        assert!(p.is_done());
        assert_eq!(p.get_results_for_fencer(&f[3]).wins, 3);
        assert_eq!(p.get_results_for_fencer(&f[2]).given, 10);
        assert_eq!(p.get_results_for_fencer(&f[1]).recieved, 14);
        assert_eq!(p.get_results_for_fencer(&f[0]).index(), 6)
    }
}
