use std::error::Error;

use clap::Parser;

use self::args::App;
use self::commands::init::init;

mod args;
pub mod bout;
pub mod commands;
pub mod fencer;
pub mod round;
pub mod file_management;
pub mod state;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::parse();

    match app.command {
        args::Command::Init => init(),
    }
}
