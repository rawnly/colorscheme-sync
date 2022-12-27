# Colorscheme
> Dead simple tool to sync colorschemes across alacritty, tmux and astronvim.

## Installation 
```sh
  cargo install colorscheme
```

Via homebrew:

```sh
  brew tap rawnly/tap
  brew installl colorscheme
```

## Usage
```
Usage: colorscheme [THEME] <COMMAND>

Commands:
  list  List available themes
  new   Generates template code for a new theme
  set   Sets the current theme
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [THEME]  name of the theme

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### Alacritty Setup
To get started make sure to have the `import` keyword defined inside your `alacritty.yml`

```yaml
import:
  # your imports here.
  # it can be empty
  - path/to/theme.yml
```
> NOTE: colorscheme replaces the last import with the path to the new theme

### Tmux Setup 
Paste the following snippet somewhere inside your `.tmux.conf`

```sh
# <THEME
{{YOUR_THEME_IMPORT_OR_TPM_PLUGIN_HERE}}
# THEME>
```

##### Example 
```sh
# <THEME
source ~/.tmux/themes/tokyonight/moon.tmux
# THEME>

run '~/.tmux/plugins/tpm/tpm'
```

### Add a new theme
Once you run `colorscheme` for the first time, it will initialize a configuration file inside `$XDG_CONFIG_HOME/.config/colorscheme/colorscheme.json`
Run `colorscheme new <theme-name>` to add a template entry to it.
This will add the following to the config file:
```jsonc
  {
    "name": "<theme-name>",
    "config": {
      // name of the astronvim theme
      "astronvim": null,
      // path to the alacritty theme
      "alacritty": null,

      // tmux configuration (default is null)
      "tmux": {
        "path": "path to the tmux theme",
        "is_plugin": false,
        // use with `is_plugin: true` to pass options to the TPM plugin
        "options": []
      }
    }
  }
```

