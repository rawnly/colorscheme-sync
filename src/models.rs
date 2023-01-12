use crate::{error::Error, fsutil::*, Result};
use anyhow::anyhow;
use home::home_dir;
use regex::Regex;
use serde_yaml::Value as YamlValue;
use std::fs;

pub type NeovimTheme = String;
pub type AlacrittyTheme = String;

#[derive(serde::Serialize, Default, serde::Deserialize, Clone)]
pub struct Theme {
    pub name: String,
    pub config: ThemeConfig,
}

impl Theme {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            config: ThemeConfig::default(),
        }
    }

    pub fn set_neovim(&self) -> Result<()> {
        let re = Regex::new(r#"colorscheme\s=\s"(?P<c>.*)","#).map_err(|e| anyhow!(e))?;

        let filepath = home_dir()
            .expect("cannot retrive homedir")
            .join(".config/nvim/lua/user/init.lua");

        if let Some(theme) = self.config.astronvim.clone() {
            let content = fs::read_to_string(&filepath)?;
            let after = re.replace_all(&content, format!(r#"colorscheme = "{}","#, theme));

            fs::write(&filepath, after.to_string())?;
        }

        Ok(())
    }

    pub fn set_alacritty(&self) -> Result<()> {
        let filepath = home_dir()
            .expect("cannot retrive homedir")
            .join(".config/alacritty/alacritty.yml");

        let config = read_yaml(&filepath)?;
        let imports = match config.get("import") {
            Some(YamlValue::Sequence(imports)) => imports,
            _ => {
                return Err(Error::InvalidConfig(
                    "invalid import type: expected a sequence",
                ));
            }
        };

        let old_theme = imports.last();

        if let Some(old_theme) = old_theme {
            if let Some(n) = self.config.alacritty.clone() {
                let mut contents = fs::read_to_string(&filepath)?;
                contents = contents.replace(old_theme.as_str().unwrap(), &n);

                fs::write(&filepath, contents)?;
            }
        }

        Ok(())
    }

    pub fn set_tmux(&self) -> Result<()> {
        let re = Regex::new(r#"#\s<THEME\n(?P<theme>.*)\n?(?P<options>.*)\n?#\sTHEME>"#).unwrap();

        let template = r#"# <THEME
{{source}}
# THEME>"#;

        let filepath = home_dir()
            .expect("cannot retrive homedir")
            .join(".tmux.conf");

        if let Some(TmuxTheme {
            path,
            is_plugin,
            options,
        }) = self.config.tmux.clone()
        {
            let mut content = fs::read_to_string(&filepath)?;

            let mut theme_string = format!("source {}", path);

            if is_plugin {
                theme_string = format!("set -g @plugin '{}'", path);

                for (key, value) in options {
                    let option = format!("set -g {} {}", key, value);
                    theme_string = format!("{}\n{}", theme_string, option);
                }
            }

            let new_theme = template.replace("{{source}}", &theme_string);

            content = re.replace_all(&content, new_theme).to_string();

            fs::write(&filepath, &content)?;
        }

        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TmuxTheme {
    /// name or path to the plugin
    pub path: String,
    /// specifies if it is a tmux plugin (tpm) or a file that needs to be sourced
    pub is_plugin: bool,
    /// extra options for tpm plugins
    pub options: Vec<(String, String)>,
}

impl TmuxTheme {
    pub fn new(path: &str) -> Self {
        Self {
            is_plugin: false,
            path: path.to_string(),
            options: Vec::default(),
        }
    }
}

#[derive(serde::Serialize, Default, serde::Deserialize, Clone)]
pub struct ThemeConfig {
    pub astronvim: Option<NeovimTheme>,
    pub alacritty: Option<AlacrittyTheme>,
    pub tmux: Option<TmuxTheme>,
}

pub type Themes = Vec<Theme>;

impl File for Themes {}
