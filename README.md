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

## Adapters

Following adapters are officially supported.

- [Shell](./adapters/shell)
- [Telegram](./adapters/telegram) (Try Live Demo: https://t.me/YozukBot)

<div align="center">
<img alt="Telegram screenshot" src="images/chat.png" />
</div>

## Skills

Yozuk manages its commands by modules called [skills](./skills).

You can enable or disable each skill at build time. Disabling unneeded skills is helpful in reducing build time, startup time, executable size and command misrecognitions.

## Build Requirements

- **Rust toolchain**: 1.58.0 or later
- **CMake**: 3.12 or later
