# youtube_live_alert

Regularly checks if a YouTube live is being streamed by the specified user and sends a notification to Discord if yes.

## Config

`./conf/config.json`

Example:

```json
{
    "name": "Mike (human-readable value here)",
    "username": "@mike_123",
    "discord_url": "https://discord.com/api/webhooks/12345/abcde",
    "timeout_sec": 10,
    "interval_sec": 60
}
```

## Usage

```bash
docker compose build
docker compose up -d
```

