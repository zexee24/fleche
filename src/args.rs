use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(version)]
pub(super) struct App {
    #[clap(flatten)]
    pub(super) global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub(super) command: Command,
}

#[derive(Subcommand)]
pub(super) enum Command {
    Init,
}

#[derive(Args, Clone)]
pub(super) struct GlobalOpts {
    #[clap(long, short, global = true)]
    pub name: Option<String>,
}
