use crate::{
    config::{self, CONFIG_FILE, XDG_PREFIX},
    error,
    fsutil::*,
    models::{Theme, ThemeConfig, Themes},
    NewOptions, Result,
};
use colored::*;

pub fn list() -> Result<()> {
    let config = config::get_config_path(XDG_PREFIX, CONFIG_FILE).unwrap();
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
    let config = config::get_config_path(XDG_PREFIX, CONFIG_FILE).unwrap();
    let mut content = Themes::load(config.as_path())?;

    let template_theme = Theme::new(&options.name);

    content.push(template_theme);
    content.write(config.as_path())?;

    println!("Theme {} added", options.name);

    Ok(())
}

pub fn set_theme(name: &str) -> Result<()> {
    let config = config::get_config_path(XDG_PREFIX, CONFIG_FILE).unwrap();
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

            println!("{} theme updated successfully", "ASTRONVIM".bold().blue());
        }

        if alacritty.is_some() {
            theme.set_alacritty()?;

            println!("{} theme updated successfully", "ALACRITTY".bold().blue());
        }

        if tmux.is_some() {
            theme.set_tmux()?;

            println!("{} theme updated successfully", "TMUX".bold().blue());
        }

        return Ok(());
    }

    Err(error::Error::ThemeNotFound)
}
