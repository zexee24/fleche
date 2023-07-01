use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::bout::{Bout, BoutScore};
use crate::fencer::Fencer;

use super::Round;

#[derive(Clone, Serialize, Deserialize)]
pub struct Group(pub Vec<Box<dyn Round>>);

impl Group {
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

    fn new(xs: Vec<Box<dyn Round>>) -> Group {
        Group(xs)
    }
}
