# yozuk-telegram

[![GitHub license](https://img.shields.io/github/license/yozuk/yozuk.svg)](https://github.com/yozuk/yozuk/blob/main/LICENSE)
[![Telegram: YozukBot](https://img.shields.io/badge/Telegram-@YozukBot-blue?logo=telegram)](https://t.me/YozukBot)

A server program to run Yozuk as a telegram bot.
Please refer [Telegram Bots](https://core.telegram.org/bots) and [Teloxide](https://github.com/teloxide/teloxide) for more information.

Live Demo: https://t.me/YozukBot

![Telegram Screenshot](chat.png)

## Starting service

```bash
export TELOXIDE_TOKEN=[Bot Token]
export PORT=8080
yozuk-telegram --webhook=http://example.com/
```

## Docker build

```bash
cd yozuk
docker build . -f adapters/telegram/Dockerfile
```