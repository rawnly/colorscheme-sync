#!/usr/bin/env node

import fs from 'node:fs'
import os from 'node:os'
import { load } from 'js-yaml'
import path from 'node:path'
import yaml from 'yaml'
import fallbackThemes from '../theme-config.json'

interface ResultSuccess<T> { data: T; ok: true }
interface ResultFailure<E> { error: E, ok: false }
type Result<T, E> = ResultSuccess<T> | ResultFailure<E>;

async function toResult<T, E>(promise: () => Promise<T>, toError: (error: unknown) => E): Promise<Result<T, E>> {
  try {
    const data = await promise()

    return {
      data,
      ok: true
    }
  } catch (error) {
    return {
      ok: false,
      error: toError(error)
    }
  }
}

async function readFile(p: string) {
  return fs.promises.readFile(path.join(os.homedir(), p), 'utf-8')
}

function toError(err: unknown): Error {
  if (err instanceof Error) return err
  return new Error(err as any)
}

function parseJson<T>(data: string): Result<T, Error> {
  try {
    const d = JSON.parse(data)

    return {
      ok: true,
      data: d
    }
  } catch (error) {
    return {
      ok: false,
      error: toError(error)
    }
  }
}

// maybe the worst thing i've ever done :/
async function getThemes(): Promise<Theme[]> {
  const c1 = await toResult(() => readFile('.colorschemes.json'), toError)
  const c2 = await toResult(() => readFile('.config/colorschemes/themes.json'), toError)

  const colorschemes = c1.ok ? c1.data : c2.ok ? c2.data : undefined

  if (!colorschemes) {
    return fallbackThemes
  }

  const json = parseJson<Theme[]>(colorschemes)

  if (!json.ok) {
    return fallbackThemes
  }

  return json.data
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

function printHelp() {
  console.log('Colorscheme')
  console.log()
  console.log(`Usage: 
      $ colorscheme <theme-name>`)
}

async function main() {
  const [theme_name = 'nightfox'] = process.argv.slice(2);

  if (['--help', '-h', 'help'].includes(theme_name)) {
    printHelp()
    return
  }

  const themes = await getThemes()
  const theme = themes.find(t => t.name === theme_name)

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
