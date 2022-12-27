use crate::{
    error,
    fsutil::*,
    models::{Theme, ThemeConfig, Themes},
    NewOptions, Result,
};
use colored::*;

pub fn list() -> Result<()> {
    let config = get_config_file()?;
    let content = Themes::load(config.as_path())?;

    if content.is_empty() {
        println!("No themes installed.");
        return Ok(());
    }

    content
        .iter()
        .for_each(|theme| println!("> {}", theme.name.yellow()));

    Ok(())
}

pub fn add(options: NewOptions) -> Result<()> {
    let config = get_config_file()?;
    let mut content = Themes::load(config.as_path())?;

    let template_theme = Theme::new(&options.name);

    content.push(template_theme);
    content.write(config.as_path())?;

    println!("Theme {} added", options.name);

    Ok(())
}

pub fn set_theme(name: &str) -> Result<()> {
    let config = get_config_file()?;
    let content = Themes::load(config.as_path())?;

    if content.is_empty() {
        println!("No themes installed.");
        return Ok(());
    }

    let theme = content.iter().find(|theme| theme.name == *name);

    if let Some(theme) = theme {
        let ThemeConfig {
            astronvim,
            alacritty,
            tmux,
        } = theme.clone().config;

        if astronvim.is_some() {
            theme.set_neovim()?;

            println!(
                "{} -> {} theme successfully set",
                "ASTRONVIM".bold().blue(),
                theme.name.yellow()
            );
        }

        if alacritty.is_some() {
            theme.set_alacritty()?;

            println!(
                "{} -> {} theme successfully set",
                "ALACRITTY".bold().blue(),
                theme.name.yellow()
            );
        }

        if tmux.is_some() {
            theme.set_tmux()?;

            println!(
                "{} -> {} theme successfully set",
                "TMUX".bold().blue(),
                theme.name.yellow()
            );
        }

        return Ok(());
    }

    Err(error::Error::ThemeNotFound)
}
