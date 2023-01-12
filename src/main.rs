use clap::{CommandFactory, Parser};
use colorscheme::{
    actions,
    config::{self, CONFIG_FILE, XDG_PREFIX},
    error::Error,
    fsutil::*,
    models::Themes,
    Action, Args,
};

fn main() -> colorscheme::Result<()> {
    let args = Args::parse();

    // get config path
    let mut config_path = config::get_config_path(XDG_PREFIX, CONFIG_FILE);

    // if the config path does not exists
    // create a new empty config file at: `$XDG_CONFIG_HOME/{XDG_PREFIX}/{CONFIG_FILE}`
    if let None = config_path {
        config_path = config::get_new_config_path(XDG_PREFIX, CONFIG_FILE);

        // if the path cannot be built for some reason throw an error
        if config_path.is_none() {
            return Err(Error::Custom("Could not create config file."));
        }

        let themes = Themes::new();

        // safe unwrap due to the previous check
        themes.write(config_path.unwrap().as_path())?;
    }

    if let Some(theme) = args.theme {
        return actions::set_theme(&theme);
    }

    if let Some(action) = args.action {
        match action {
            Action::List => actions::list()?,
            Action::New(options) => actions::add(options)?,
            Action::Set(options) => actions::set_theme(&options.name)?,
        }

        return Ok(());
    }

    // print help
    Args::command().print_help()?;

    Ok(())
}
