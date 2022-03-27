<div align="center">
<img alt="yozuk" src="images/yozuk.png" width="280" />
  
[![GitHub license](https://img.shields.io/github/license/yozuk/yozuk.svg)](https://github.com/yozuk/yozuk/blob/main/LICENSE)
[![Rust](https://github.com/yozuk/yozuk/actions/workflows/rust.yml/badge.svg)](https://github.com/yozuk/yozuk/actions/workflows/rust.yml)
[![Telegram: YozukBot](https://img.shields.io/badge/Telegram-@YozukBot-blue?logo=telegram)](https://t.me/YozukBot)
</div>

- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Online Demo](#online-demo)
  - [Pre-built binaries](#pre-built-binaries)
  - [Build from Source](#build-from-source)
- [Skills](#skills)
  - [Examples](#examples)
  - [Config](#config)
- [Credits](#credits)

## Overview

**Yozuk** is an assistant bot designed for helping programmers with trivial tasks such as UUID generation.

<img alt="Yozuk CLI demo animation" src="images/zuk.gif" width="520" />

Unlike normal command-line tools, it uses a simple NLP approach to infer the meaning of your requests, so you don't have to remember the exact syntax of commands. This feature also makes Yozuk suitable for chatbots.

### Online Demo: 👉 https://yozuk.com

### Yozuk is:

- 📦 **Portable:** Runs as a single-file executable.
- 🏝️ **Stand-alone:** No internet access or external database needed.
- 🤖 **Smart:** Automagically construes your requests.
- 📟 **Handy:** Provides simple text-based interface accessible from any device.
- 🔒 **Privacy-first:** Never expose any data from your computer.
- 🚀 **Fast:** Written in Rust, compiles to native code.
- 🛠️ **Customizable:** Enabling / disabling specific commands with feature flags.

## Getting Started

### Online Demo

 - Website https://yozuk.com
 - Telegram Bot (https://t.me/YozukBot)

> Note that they are intended for demo purposes. Do not send sensitive data.

### Pre-built binaries

[Pre-built binaries](https://github.com/yozuk/yozuk/releases) for x64 Windows, Linux and macOS are available.

### Build from Source

- [zuk](./adapters/zuk) (Command-line interface)
- [Telegram](./adapters/telegram)

#### Build Requirements

- **Rust toolchain**: 1.58.0 or later
- **CMake**: 3.12 or later

## Skills

Yozuk manages its commands by modules called [skills](./skills).

### Examples

| skill | description | example queries |
| - | - | - |
| [`calc`](./skills/calc) | Simple calculator | `1 + 2` `(0.1 + 0.2) / 0.3` |
| [`uuid`](./skills/uuid) | UUID generator | `uuid` `generate 5 UUIDs` |
| [`nanoid`](./skills/nanoid) | NanoID generator | `nanoid` `generate 5 NanoIDs` |
| [`base64`](./skills/base64) | Base64 encoder/decoder | `"Hello 世界" to base64` `cXVpY2sgYnJvd24g8J+mig==` |
| [`punycode`](./skills/punycode) | Punycode encoder/decoder | `😻.example.com` `xn--hj8h.com` |
| [`lipsum`](./skills/lipsum) | Dummy text generator | `lipsum` `Lorem ipsum 150 words` |
| [`dice`](./skills/dice) | Dice roller | `2d6` `2d6 * 10 + 1d100` |
| [`digest`](./skills/digest) | Hash generator | `md5` `sha1 sha-256 Keccak-256` |

You can enable or disable each skill at build time. Disabling unneeded skills is helpful in reducing build time, startup time, executable size and command misrecognitions.

### Config

Some skills have run-time options. You can configure them with a TOML file.

```toml
[skills.yozuk-skill-dice]
secure = true

[skills.yozuk-skill-lipsum]
custom_text = "Fortune, good night: smile once more; turn thy wheel!"
```

```bash
zuk -c config.toml roll dice
```

## Credits

Yozuk was inspired by the following projects:

- [DevToys](https://github.com/veler/DevToys)

[![xkcd: tar](https://imgs.xkcd.com/comics/tar.png)](https://xkcd.com/1168/)
