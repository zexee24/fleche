use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::bout::{Bout, BoutScore};
use crate::fencer::Fencer;

use super::poule::Poule;
use super::Round;

#[derive(Clone, Serialize, Deserialize)]
pub struct Group<T: Round>(pub Vec<T>);

impl Round for Group<Poule> {
    fn get_fencers(&self) -> Vec<Rc<Fencer>> {
        self.0.iter().flat_map(|x| x.get_fencers()).collect()
    }

    fn add_results(&mut self, b: BoutScore) {
        self.0.iter_mut().for_each(|x| x.add_results(b.clone()))
    }

    fn is_done(&self) -> bool {
        self.0.iter().all(|x| x.is_done())
    }

    fn get_bouts(&self) -> Vec<Rc<Bout>> {
        self.0.iter().flat_map(|x| x.get_bouts()).collect()
    }

    fn new(fencers: Vec<Rc<Fencer>>) -> Self
    where
        Self: Sized,
    {
        Group::new_generic(vec![Poule::new(fencers)])
    }
}

impl<T: Round> Group<T> {
    pub fn new_generic(xs: Vec<T>) -> Group<T> {
        Group(xs)
    }
}
