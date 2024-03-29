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

**Yozuk** is a bot designed for helping programmers with trivial tasks such as UUID generation, digest calculation and timestamp conversion.

- **Human-friendly**: With a combination of machine-learning and pattern-matching NLP, Yozuk infers the meaning of your requests so you don't have to remember the exact syntax of commands.

- **Transparent:** Fully open-source, no external dependency, no internet access and no tracking. Your data is completely under your control.

- **Cross-platform:** In addition to WebAssembly support (Web browser and [WASI](https://wasi.dev/)), Yozuk runs natively on Windows, macOS, Linux, Android and iOS.

## What can Yozuk do?

For example...

- UUID generation

  ```
  >>> generate 3 uuids
  7a4ef819-c6b1-4e12-a446-d108db66bd9d
  b4c3ae2d-601b-416f-bf06-ef4540206d2f
  dce35c4e-974f-4bc7-8a5f-7e2f0d0820ba
  ```

- Calculation

  ```
  >>> (12345 + 43) * sqrt(5)
  27700.41010526739852
  ```

- Dummy text generation

  ```
  >>> 25 words lorem ipsum
  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis.
  ```

- Digest calculation

  ```
  >>> "Sphinx of black quartz, judge my vow!" to sha384
  0dd8b5542f71641b2bfe5b55c8f3e1e092777b7f8b6b0eae14089f7af3a67d57093a7c19c21d003c11e8cceae6b9e29e
  ```

- Base64 decoding

  ```
  >>> 8J+MuPCfjLzwn42E8J+Mvw==
  🌸🌼🍄🌿
  ```

- ...and so on!

  The comprehensive [skill list is here](https://docs.yozuk.com/docs/skills/).

## Getting Started

### Documentation

https://docs.yozuk.com

### Online Demo

 - Web App: https://yozuk.com
 - Telegram Bot: [![Telegram: YozukBot](https://img.shields.io/badge/Telegram-@YozukBot-blue?logo=telegram)](https://t.me/YozukBot)
 - Discord Bot: [![Discord: Yozuk#6060](https://img.shields.io/badge/Bot-Yozuk%236060-white?color=5865F2&logo=discord&logoColor=white)](https://discord.com/api/oauth2/authorize?client_id=989503720473636914&permissions=100352&scope=bot)

### Command-Line App

Yozuk works as a standalone single executable called `zuk`.

[![asciicast](https://asciinema.org/a/510703.svg)](https://asciinema.org/a/510703)

#### Gitpod

 [![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/yozuk/yozuk)

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

