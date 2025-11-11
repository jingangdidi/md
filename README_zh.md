# md
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jingangdidi/md/blob/master/LICENSE)

[English readme](https://github.com/jingangdidi/md/blob/master/README.md)

**A command-line tool convert markdown file to single html file**

**轻量级命令行工具，markdown转html，无需安装，仅一个可执行文件**

## 👑 特点
- 💪​ 单个可执行文件（~700Kb），无需安装
- 🎨​ 基于[markdown-it](https://github.com/markdown-it/markdown-it)和[highlight.js](https://github.com/highlightjs/highlight.js)
- 1️⃣​ 保存为单个HTML文件

## 🚀 使用说明
**1. 下载预编译的可执行文件**

  [latest release](https://github.com/jingangdidi/md/releases)

**2. 命令行调用**

```
./mkd -f README.md
```
生成`README.html`

## 🛠 从源码编译
```
git clone https://github.com/jingangdidi/md.git
cd md
cargo build --release
```

## 🚥 命令行参数
```
Usage: mkd -f <file> [-l <language>] [-w <width>] [-o <outpath>]

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

## ⏰ 更新记录
- [2025.11.11] release [v0.1.1](https://github.com/jingangdidi/md/releases/tag/v0.1.1)
  - 🛠修复：将`\`转义为`\\`
- [2025.11.03] release [v0.1.0](https://github.com/jingangdidi/md/releases/tag/v0.1.0)
