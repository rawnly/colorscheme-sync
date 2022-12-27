pub mod actions;
pub mod error;
pub mod fsutil;
pub mod models;

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(clap::Parser, Debug, Clone)]
pub struct NewOptions {
    #[clap(long, short)]
    pub name: String,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Action {
    /// List available themes
    List,

    /// Generates template code for a new theme
    New(NewOptions),

    Set(NewOptions),
}

#[derive(clap::Parser, Clone, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}
