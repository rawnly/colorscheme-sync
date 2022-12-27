use home::home_dir;
use std::path::PathBuf;

/// Name for XDG_PREFIX
pub const XDG_PREFIX: &'static str = "colorscheme";

/// Config file name
pub const CONFIG_FILE: &'static str = "colorscheme.json";

/// Get the path for creating a new config file
/// trying to use `$XDG_CONFIG_HOME/{prefix}/{filename}`
#[cfg(not(windows))]
pub fn get_new_config_path(prefix: &str, filename: &str) -> Option<PathBuf> {
    xdg::BaseDirectories::with_prefix(prefix)
        .ok()
        .and_then(|base| base.place_config_file(filename).ok())
}

/// Get the path for creating a new config file on windows
#[cfg(windows)]
pub fn get_new_config_path(prefix: &str, filename: &str) -> Option<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join(&format!("{}\\{}", prefix, filename)))
        .filter(|p| p.exists())
}

/// Get the location of the first found config file paths
/// according to the following order:
///
/// 1. $XDG_CONFIG_HOME/{prefix}/{filename}.json
/// 2. $XDG_CONFIG_HOME/{prefix}.json
/// 3. $HOME/.config/{prefix}/{filename}
/// 4. $HOME/.{prefix}.json
#[cfg(not(windows))]
pub fn get_config_path(prefix: &str, filename: &str) -> Option<PathBuf> {
    xdg::BaseDirectories::with_prefix(prefix)
        .ok()
        // Search for case n. 1
        .and_then(|xdg| xdg.find_config_file(filename))
        .or_else(|| {
            xdg::BaseDirectories::new()
                .ok()
                // Search for case n. 2
                .and_then(|fallback| fallback.find_config_file(&format!("{}.json", prefix)))
        })
        .or_else(|| {
            if let Some(home_path) = home_dir() {
                // Search for case n. 3
                let fallback = home_path.join(".config/colorscheme").join(filename);

                if fallback.exists() {
                    return Some(fallback);
                }

                // Search for case n. 4
                let fallback = home_path.join(&format!(".{}.json", prefix));

                if fallback.exists() {
                    return Some(fallback);
                }
            }

            None
        })
}

/// Get the location of the config file on windows
#[cfg(windows)]
pub fn get_config_path(prefix: &str, filename: &str) -> Option<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join(&format!("{}\\{}", prefix, filename)))
        .filter(|p| p.exists())
}
