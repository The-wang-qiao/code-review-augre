[![Build and Test](https://github.com/The-wang-qiao/code-review-augre/actions/workflows/build.yml/badge.svg)](https://github.com/The-wang-qiao/code-review-augre/actions/workflows/build.yml)
[![Version](https://img.shields.io/crates/v/augre.svg)](https://crates.io/crates/augre)
[![Crates.io](https://img.shields.io/crates/d/augre?label=crate)](https://crates.io/crates/augre)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# code-review-augre

A comprehensive local diff code review tool powered by LLM, CodeLlama or OpenAI.

## Binary Usage

### Install

Windows:

```powershell
$ iwr https://github.com/The-wang-qiao/code-review-augre/releases/latest/download/augre_x86_64-pc-windows-gnu.zip
$ Expand-Archive augre_x86_64-pc-windows-gnu.zip -DestinationPath C:\Users\%USERNAME%\AppData\Local\Programs\augre
```

Mac OS (Apple Silicon):

```bash
$ curl -LO https://github.com/The-wang-qiao/code-review-augre/releases/latest/download/augre_aarch64-apple-darwin.zip
$ unzip augre_aarch64-apple-darwin.zip -d /usr/local/bin
$ chmod a+x /usr/local/bin/augre
```

Linux:

```bash
$ curl -LO https://github.com/The-wang-qiao/code-review-augre/releases/latest/download/augre_x86_64-unknown-linux-gnu.zip
$ unzip augre_x86_64-unknown-linux-gnu.zip -d /usr/local/bin
$ chmod a+x /usr/local/bin/augre
```

Cargo:

```bash
$ cargo install augre
```

### Help Docs

```bash
$ augre -h
Usage: augre [OPTIONS] [COMMAND]

Commands:
  review  Performs a code review of the current `git diff HEAD^`
  ask     Gives a response to the specified prompt
  stop    Stop all of the background services
  help    Print this message or the help of the given subcommand(s)
