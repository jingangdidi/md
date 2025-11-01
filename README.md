# md
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jingangdidi/md/blob/master/LICENSE)

[中文文档](https://github.com/jingangdidi/md/blob/master/README_zh.md)

**A command-line tool convert markdown file to single html file**

## 👑 Features
- 💪​ Single-file executable (~700Kb) - no installation required
- 🎨​ Based on [markdown-it](https://github.com/markdown-it/markdown-it) and [highlight.js](https://github.com/highlightjs/highlight.js)
- 1️⃣​ Save as single HTML file

## 🚀 Usage
**1. download a pre-built binary**

  [latest release](https://github.com/jingangdidi/md/releases)

**2. convert markdown to html**
```
./md -f README.md
```
This will create `README.html`

## 🛠 Building from source
```
git clone https://github.com/jingangdidi/md.git
cd md
cargo build --release
```

## 🚥 Arguments
```
Usage: md -f <file> [-l <language>] [-w <width>] [-o <outpath>]

render markdown, save as html file

Options:
  -f, --file        markdown file, multiple files separated by commas
  -l, --language    show what language when copy code, support: en, es, pt-BR, fr, de, ja, ko, ru, zh, zh-tw, default: en
  -w, --width       width, (0, 100], default: 60
  -o, --outpath     output path, default: ./
  -h, --help        display usage information
```

## ❤️ Acknowledgements
- [markdown-it](https://github.com/markdown-it/markdown-it)
- [highlight.js](https://github.com/highlightjs/highlight.js)

## ⏰ changelog
- [2025.11.01] release v0.1.0
