use clap::Parser;
use colorscheme::{actions, Action, Args};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.action {
        Action::List => {
            actions::list()?;
        }
        Action::New(options) => {
            actions::add(options)?;
        }
        Action::Set(options) => {
            actions::set_theme(&options.name)?;
        }
    };

    Ok(())
}
