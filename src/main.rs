use clap::Parser;

use self::args::App;

mod args;
pub mod fencer;
pub mod state;
pub mod variants;

fn main() {
    let app = App::parse();

    match app.command {
        args::Command::Init => todo!(),
    }
}
