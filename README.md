<div align="center">
<img alt="yozuk" src="images/yozuk.png" height="180" />
  
[![GitHub license](https://img.shields.io/github/license/yozuk/yozuk.svg)](https://github.com/yozuk/yozuk/blob/main/LICENSE)
[![Rust](https://github.com/yozuk/yozuk/actions/workflows/rust.yml/badge.svg)](https://github.com/yozuk/yozuk/actions/workflows/rust.yml)
[![Telegram: YozukBot](https://img.shields.io/badge/Telegram-@YozukBot-blue?logo=telegram)](https://t.me/YozukBot)
</div>

## Overview

**Yozuk** is an assistant bot designed for helping programmers with trivial tasks such as UUID generation or Base64 encoding / decoding.

Unlike normal command-line tools, it uses a simple NLP approach to infer the meaning of your requests, so you don't have to remember the exact syntax of commands. This feature also makes Yozuk suitable for chatbots.

### Yozuk is:

- 📦 **Portable:** Runs as a single-file executable.
- 🏝️ **Stand-alone:** No internet access or external database needed.
- 🤖 **Smart:** Automagically construes your requests.
- 🔒 **Privacy-first:** Never expose any data from your computer.
- 🚀 **Fast:** Written in Rust, compiles to native code.
- 🛠️ **Customizable:** Enabling / disabling specific commands with feature flags.

[![xkcd: tar](https://imgs.xkcd.com/comics/tar.png)](https://xkcd.com/1168/)

## Skills

Yozuk manages its commands by modules called [skills](./skills).

### Skill examples

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

## Adapters

Following adapters are officially supported.

- [zuk](./adapters/zuk) (Command-line interface)
- [Telegram](./adapters/telegram) (Try Live Demo: https://t.me/YozukBot)

<div align="center">
<img alt="Telegram screenshot" src="images/chat.png" />
</div>

## Build Requirements

- **Rust toolchain**: 1.58.0 or later
- **CMake**: 3.12 or later
