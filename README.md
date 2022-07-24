<div align="center">
<img alt="yozuk" src="https://github.com/yozuk/yozuk/blob/main/images/yozuk.png?raw=true" width="120" />

# Yozuk

[![GitHub license](https://img.shields.io/github/license/yozuk/yozuk.svg)](https://github.com/yozuk/yozuk/blob/main/LICENSE)
[![Rust](https://github.com/yozuk/yozuk/actions/workflows/rust.yml/badge.svg)](https://github.com/yozuk/yozuk/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/yozuk/yozuk/status.svg)](https://deps.rs/repo/github/yozuk/yozuk)
[![crates.io](https://img.shields.io/crates/v/yozuk.svg)](https://crates.io/crates/yozuk)
[![WAPM package](https://wapm.io/package/yozuk/zuk/badge.svg?style=flat)](https://wapm.io/package/yozuk/zuk)
[![Telegram: YozukBot](https://img.shields.io/badge/Telegram-@YozukBot-blue?logo=telegram)](https://t.me/YozukBot)
[![Discord: Yozuk#6060](https://img.shields.io/badge/Bot-Yozuk%236060-white?color=5865F2&logo=discord&logoColor=white)](https://discord.com/api/oauth2/authorize?client_id=989503720473636914&permissions=100352&scope=bot)
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

 - Web App: https://yozuk.com
 - Telegram Bot: [![Telegram: YozukBot](https://img.shields.io/badge/Telegram-@YozukBot-blue?logo=telegram)](https://t.me/YozukBot)
 - Discord Bot: [![Discord: Yozuk#6060](https://img.shields.io/badge/Bot-Yozuk%236060-white?color=5865F2&logo=discord&logoColor=white)](https://discord.com/api/oauth2/authorize?client_id=989503720473636914&permissions=100352&scope=bot)

### Command-Line App

[![asciicast](https://asciinema.org/a/510703.svg)](https://asciinema.org/a/510703)

#### Pre-built binaries

- Windows x64 [zuk-x86_64-pc-windows-msvc.zip](https://github.com/yozuk/yozuk/releases/latest/download/zuk-x86_64-pc-windows-msvc.zip)
- Windows ARM64 [zuk-aarch64-pc-windows-msvc.zip](https://github.com/yozuk/yozuk/releases/latest/download/zuk-aarch64-pc-windows-msvc.zip)
- MacOS Intel [zuk-x86_64-apple-darwin.tar.xz](https://github.com/yozuk/yozuk/releases/latest/download/zuk-x86_64-apple-darwin.tar.xz)
- MacOS Apple Silicon [zuk-aarch64-apple-darwin.tar.xz](https://github.com/yozuk/yozuk/releases/latest/download/zuk-aarch64-apple-darwin.tar.xz)
- Linux x64 [zuk-x86_64-unknown-linux-gnu.tar.xz](https://github.com/yozuk/yozuk/releases/latest/download/zuk-x86_64-unknown-linux-gnu.tar.xz)
- Linux ARM64 [zuk-aarch64-unknown-linux-gnu.tar.xz](https://github.com/yozuk/yozuk/releases/latest/download/zuk-aarch64-unknown-linux-gnu.tar.xz)

<details>
  <summary>Other Platforms</summary>

- Linux ARMv7 [zuk-armv7-unknown-linux-gnueabihf.tar.xz](https://github.com/yozuk/yozuk/releases/latest/download/zuk-armv7-unknown-linux-gnueabihf.tar.xz)
- Linux ARM64 Android (Termux) [zuk-aarch64-linux-android.tar.xz](https://github.com/yozuk/yozuk/releases/latest/download/zuk-aarch64-linux-android.tar.xz)

</details>

#### Homebrew

```bash
brew tap yozuk/yozuk
brew install zuk
```

#### Cargo

```bash
cargo install zuk
```

#### WAPM

```bash
wapm install yozuk/zuk
```

### Libraries

#### Rust

- [yozuk](https://crates.io/crates/yozuk) [![crates.io](https://img.shields.io/crates/v/yozuk.svg)](https://crates.io/crates/yozuk)
- [yozuk-sdk](https://crates.io/crates/yozuk-sdk) [![crates.io](https://img.shields.io/crates/v/yozuk-sdk.svg)](https://crates.io/crates/yozuk-sdk)

#### TypeScript / JavaScript

- [@yozuk/yozuk-wasm](https://www.npmjs.com/package/@yozuk/yozuk-wasm) [![npm version](https://badge.fury.io/js/@yozuk%2Fyozuk-wasm.svg)](https://badge.fury.io/js/@yozuk%2Fyozuk-wasm)

## Credits

Yozuk was inspired by the following projects:

- [DevToys](https://github.com/veler/DevToys)

