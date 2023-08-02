use std::error::Error;
use std::rc::Rc;

use inquire::validator::Validation;
use strum::IntoEnumIterator;

use crate::fencer::Fencer;
use crate::round::group::Group;
use crate::round::poule::Poule;
use crate::round::{Round, RoundType};
use crate::state::State;

pub mod init;

type FencerList = Vec<Rc<Fencer>>;

pub(super) fn create_new_round(state: &mut State) -> Result<(), Box<dyn Error>> {
    let a = match inquire::Select::new(
        "Next select first round type",
        RoundType::iter().collect::<Vec<_>>(),
    )
    .prompt()?
    {
        RoundType::Poule => {
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
            split_fencers(
                state.get_fencers().to_vec(),
                (0..res).map(|_| RoundType::Poule).collect(),
            )
        }
        RoundType::Cup => todo!(),
    };
    state.add_round(a);
    Ok(())
}

pub(super) fn split_fencers(fencers: FencerList, groups: Vec<RoundType>) -> Group {
    let n = groups.len();
    let initial_split: Vec<FencerList> = (0..n).map(|_| Vec::new()).collect();
    let mut counter = 0;
    fencers.into_iter().for_each(|x| {
        initial_split[counter % n].push(x);
        counter += 1;
    });

    let pdt = groups.iter().fold(String::new(), |acc, x| format!("{}\n{}", x.to_string(), ))
    inquire::Editor::new("Split the fencers").with_predefined_text()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::round::poule::tests::mock_few_fencers;

    #[test]
    fn test_splitting_fencers() {
        let fencers = mock_few_fencers(7);
        let group = split_fencers(fencers.clone(), vec![RoundType::Poule, RoundType::Poule]);

        assert_eq!(group.0[0].get_fencers()[2], fencers[4]);
        assert_eq!(group.0[1].get_fencers()[1], fencers[3]);
    }
}
