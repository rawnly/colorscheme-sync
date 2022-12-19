import themes from '../theme-config.json'
import fs from 'node:fs'
import os from 'node:os'
import { load } from 'js-yaml'
import path from 'node:path'
import yaml from 'yaml'

function getThemes(): Theme[] {
  return themes as any
}

interface Theme {
  name: string;
  items: {
    neovim: string;
    alacritty: string;
    tmux: {
      name: string
      plugin: boolean
    }
  }
}

function main() {
  const [theme_name = 'nightfox'] = process.argv.slice(2);
  const theme = getThemes().find(t => t.name === theme_name)

  if (!theme) {
    console.warn('cannot find theme: ', theme_name);
    process.exit(0)
  }


  setNeovimTheme(theme.items.neovim)
  setAlacrittyTheme(theme.items.alacritty)
  setTmuxTheme(theme.items.tmux)
}

function setNeovimTheme(theme: string) {
  const REGEX = /colorscheme\s=\s\"(?<color>.*)",/g
  const filepath = path.join(os.homedir(), '.config/nvim/lua/user/init.lua')

  const content = fs.readFileSync(filepath, 'utf-8')
  const updated = content.replace(REGEX, `colorscheme = "${theme}",`)

  fs.writeFileSync(filepath, updated)
}

function setAlacrittyTheme(theme: string) {
  const filepath = path.join(os.homedir(), '.config/alacritty/alacritty.yml');
  const content: any = load(fs.readFileSync(filepath, 'utf-8'));

  content.import.pop()
  content.import.push(theme.replace(os.homedir(), '~'))

  const doc = new yaml.Document()
  doc.contents = content

  fs.writeFileSync(filepath, doc.toString())
}

function setTmuxTheme(theme: Theme['items']['tmux']) {
  const REGEX = /#\s<THEME\n(?<theme>.*)\n?(?<options>.*)\n?#\sTHEME>/gm

  const filepath = path.join(os.homedir(), '.tmux.conf')
  const content: any = fs.readFileSync(filepath, 'utf-8')

  const template = `# <THEME\n{{source}}\n# THEME>`
  const themeString = theme.plugin ? `set -g @plugin '${theme.name}'` : `source ${theme.name}`
  const updatedContent = content.replace(REGEX, template.replace('{{source}}', themeString))

  fs.writeFileSync(filepath, updatedContent)
}

main()
