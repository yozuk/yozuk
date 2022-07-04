<div align="center">
<img alt="yozuk" src="https://github.com/yozuk/yozuk/blob/main/images/yozuk.png?raw=true" width="120" />

# Yozuk

[![GitHub license](https://img.shields.io/github/license/yozuk/yozuk.svg)](https://github.com/yozuk/yozuk/blob/main/LICENSE)
[![Rust](https://github.com/yozuk/yozuk/actions/workflows/rust.yml/badge.svg)](https://github.com/yozuk/yozuk/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/yozuk/yozuk/status.svg)](https://deps.rs/repo/github/yozuk/yozuk)
[![WAPM package](https://wapm.io/package/yozuk/zuk/badge.svg?style=flat)](https://wapm.io/package/yozuk/zuk)
[![Telegram: YozukBot](https://img.shields.io/badge/Telegram-@YozukBot-blue?logo=telegram)](https://t.me/YozukBot)
</div>

## Overview

**Yozuk** is a chatbot designed for helping programmers with trivial tasks such as UUID generation, hash calculation and timestamp conversion.

- **Knows what you want**: With a combination of machine-learning and pattern-matching NLP, Yozuk infers the meaning of your requests so you don't have to remember the exact syntax of commands.

- **Provides full transparency:** Fully open-source, no external dependency, no internet access and no tracking. Your data is completely under your control.

- **Runs everywhere:** Not to mention WebAssembly support (Web browser and [WASI](https://wasi.dev/)), Yozuk runs natively on Windows, macOS, Linux, Android and iOS.

<img alt="Yozuk CLI demo animation" src="https://github.com/yozuk/yozuk/blob/main/images/zuk.gif?raw=true" width="520" />

## Getting Started

### Documentation

https://docs.yozuk.com

### Online Demo

 - Web App https://yozuk.com
 - Telegram Bot (https://t.me/YozukBot)

### Pre-built binaries

[Pre-built binaries](https://github.com/yozuk/yozuk/releases) for x64 Windows, Linux and macOS are available.

### Homebrew

```bash
brew tap yozuk/yozuk
brew install zuk
```

### Cargo

```bash
cargo install zuk
```

### WAPM

```bash
wapm install yozuk/zuk
```

### Build from Source

- [zuk](./zuk) (Command-line interface)
- [Telegram](https://github.com/yozuk/yozuk-telegram)

#### Build Requirements

- **Rust toolchain**: 1.60.0 or later
- **CMake**: 3.12 or later

## Credits

Yozuk was inspired by the following projects:

- [DevToys](https://github.com/veler/DevToys)

