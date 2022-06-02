<div align="center">
<img alt="yozuk" src="https://github.com/yozuk/yozuk/blob/main/images/yozuk.png?raw=true" width="280" />
  
[![GitHub license](https://img.shields.io/github/license/yozuk/yozuk.svg)](https://github.com/yozuk/yozuk/blob/main/LICENSE)
[![Rust](https://github.com/yozuk/yozuk/actions/workflows/rust.yml/badge.svg)](https://github.com/yozuk/yozuk/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/yozuk/yozuk/status.svg)](https://deps.rs/repo/github/yozuk/yozuk)
[![WAPM package](https://wapm.io/package/yozuk/zuk/badge.svg?style=flat)](https://wapm.io/package/yozuk/zuk)
[![Telegram: YozukBot](https://img.shields.io/badge/Telegram-@YozukBot-blue?logo=telegram)](https://t.me/YozukBot)
</div>

## Overview

**Yozuk** is an assistant bot designed for helping programmers with trivial tasks such as UUID generation, hash calculation and timestamp conversion.
Unlike normal command-line tools, it uses a simple NLP approach to infer the meaning of your requests, so you don't have to remember the exact syntax of commands.

<img alt="Yozuk CLI demo animation" src="https://github.com/yozuk/yozuk/blob/main/images/zuk.gif?raw=true" width="520" />

### Advantages

- **Transparent:** Fully open-source, no external dependency, no internet access and no tracking. Your data is completely under your control.

- **Portable:** Not to mention WebAssembly support, Yozuk runs natively on Windows, macOS, Linux, Android and iOS.

- **Flexible:** Yozuk’s simple text-based interface is lightweight and easy to extend. You can stay in touch with it from any device.

## Getting Started

### Documentation

https://yozuk.com

### Online Demo

 - Web App https://app.yozuk.com
 - Telegram Bot (https://t.me/YozukBot)

### Pre-built binaries

[Pre-built binaries](https://github.com/yozuk/yozuk/releases) for x64 Windows, Linux and macOS are available.

### WAPM

```bash
wapm install yozuk/zuk
```

### Cargo

```bash
cargo install zuk
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

