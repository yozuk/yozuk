# yozuk-slack
Yozuk slack bot

## Starting Server

```
yozuk-slack --addr 127.0.0.1 --port 8080 --token [BOT_USER_TOKEN]

or

export PORT=8080
export SLACK_TOKEN=[BOT_USER_TOKEN]
yozuk-slack --addr 127.0.0.1
```

## Slack Bot Configuration

### Scopes

`yozuk-slack` requires the following permission scopes.

- `app_mentions:read`
- `chat:write`
- `file:write`
- `im:history`

### Event Subscriptions

`yozuk-slack` receives requests via the [Slack Events API](https://api.slack.com/apis/connections/events-api).

You have to add the following bot user events.

- `app_mention`
- `message.im`