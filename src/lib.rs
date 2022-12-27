pub mod actions;
pub mod config;
pub mod error;
pub mod fsutil;
pub mod models;

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(clap::Parser, Debug, Clone)]
pub struct NewOptions {
    pub name: String,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Action {
    /// List available themes
    List,

    /// Generates template code for a new theme
    New(NewOptions),

    /// Sets the current theme
    Set(NewOptions),
}

#[derive(clap::Parser, Clone, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,

    /// name of the theme
    pub theme: Option<String>,
}
