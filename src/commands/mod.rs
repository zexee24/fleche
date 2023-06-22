use std::error::Error;

use strum::IntoEnumIterator;

use crate::round::RoundTypes;
use crate::state::State;

pub mod init;

pub(super) fn choose_next_round(state: &mut State) -> Result<(), Box<dyn Error>> {
    match inquire::Select::new(
        "Next select first round type",
        RoundTypes::iter().collect::<Vec<_>>(),
    )
    .prompt()?
    {
        RoundTypes::Poule => todo!(),
        RoundTypes::Cup => todo!(),
    }
}
