# yozuk-telegram

A server program to run Yozuk as a telegram bot.
Please refer [Telegram Bots](https://core.telegram.org/bots) and [Teloxide](https://github.com/teloxide/teloxide) for more information.

Live Demo: https://t.me/YozukBot

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