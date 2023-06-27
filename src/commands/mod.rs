use std::error::Error;
use std::rc::Rc;

use inquire::validator::Validation;
use strum::IntoEnumIterator;

use crate::fencer::Fencer;
use crate::round::group::Group;
use crate::round::poule::Poule;
use crate::round::{Round, RoundTypes};
use crate::state::State;

pub mod init;

pub(super) fn choose_next_round(state: &mut State) -> Result<(), Box<dyn Error>> {
    let a = match inquire::Select::new(
        "Next select first round type",
        RoundTypes::iter().collect::<Vec<_>>(),
    )
    .prompt()?
    {
        RoundTypes::Poule => {
            let default = state.fencer_amout() / 7;
            let min_allowed = (state.fencer_amout() / 2) as u32;
            let res = inquire::Text::new("Enter the amount of poules")
                .with_default(&default.to_string())
                .with_validator(move |x: &str| {
                    match x
                        .parse::<u32>()
                        .map(|x| min_allowed >= x && x > 0)
                        .unwrap_or(false)
                    {
                        true => Ok(Validation::Valid),
                        false => Ok(Validation::Invalid("Is not a valid number".into())),
                    }
                })
                .prompt()?
                .parse::<u32>()?;
            split_fencers::<Poule>(state.get_fencers().to_vec(), res)
        }
        RoundTypes::Cup => todo!(),
    };
    state.add_round(Box::new(a));
    Ok(())
}

pub(super) fn split_fencers<T: Round>(fencers: Vec<Rc<Fencer>>, groups: u32) -> Group<T> {
    let mut gl: Vec<Vec<Rc<Fencer>>> = (0..groups).map(|_| Vec::new()).collect();
    let loopable = fencers.into_iter();
    let mut counter = 0;
    for x in loopable {
        gl[counter].push(x);
        if counter == gl.len() - 1 {
            counter = 0;
        } else {
            counter += 1;
        }
    }
    Group::new_generic(gl.into_iter().map(|x| T::new(x)).collect::<Vec<T>>())
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::round::poule::tests::mock_few_fencers;

    #[test]
    fn test_splitting_fencers(){
        let fencers = mock_few_fencers(7);
        let group = split_fencers::<Poule>(fencers.clone(),2);

        assert_eq!(group.0[0].get_fencers()[2], fencers[4]);
        assert_eq!(group.0[1].get_fencers()[1], fencers[3]);
    }
}
