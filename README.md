<div align="center">
<img alt="yozuk" src="images/yozuk.png" height="180" />
</div>

## Overview

**Yozuk** is an assistant bot designed for helping programmers with trivial tasks such as UUID generation.

Unlike normal command-line tools, it uses a simple NLP approach to infer the meaning of your requests, so you don't have to remember the exact syntax of commands. This feature also makes Yozuk suitable for chatbots.

### Yozuk is:

- 📦 **Portable:** Runs as a single-file executable.
- 🏝️ **Stand-alone:** No internet access or external database needed.
- 🔒 **Privacy-first:** Never expose any data from your computer.
- 🚀 **Fast:** Written in Rust, compiles to native code.
- 🛠️ **Customizable:** Enabling / disabling specific commands with feature flags.

## Skills

Yozuk manages its commands by modules called [skills](./skills).

### Skill examples

| skill | description | example queries |
| - | - | - |
| `calc` | Simple calculator | `1 + 2` `(0.1 + 0.2) / 0.3` |
| `uuid` | UUID generator | `uuid` `generate 5 UUIDs` |
| `nanoid` | NanoID generator | `nanoid` `generate 5 NanoIDs` |
| `base64` | Base64 encoder/decoder | `"Hello 世界" to base64` `cXVpY2sgYnJvd24g8J+mig==` |
| `punycode` | Punycode encoder/decoder | `😻.example.com` `xn--hj8h.com` |
| `lipsum` | Dummy text generator | `lipsum` `Lorem ipsum dolor sit amet,` |
| `dice` | Dummy text generator | `2d6` `2d6 * 10 + 1d100` |

You can enable or disable each skill at build time. Disabling unneeded skills is helpful in reducing build time, startup time, executable size and command misrecognitions.

## Adapters

Following adapters are officially supported.

- [Shell](./adapters/shell)
- [Telegram](./adapters/telegram) (Try Live Demo: https://t.me/YozukBot)

<div align="center">
<img alt="Telegram screenshot" src="images/chat.png" />
</div>

## Build Requirements

- **Rust toolchain**: 1.58.0 or later
- **CMake**: 3.12 or later
