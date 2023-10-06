# brevo_discord_webhook

A webhook handler to post Brevo events to Discord

## Set-up

Application needs a few environnement variables to run

- AUTHORIZED_IP_RANGES : list of IP ranges authorized to post event. Must be in the format "192.168.1.0/24,10.0.0.0/8,127.0.0.1/32" format with comma separators
- DISCORD_WEBHOOK_TOKEN : the webhook to connect too, with a full url format
- HOST : ip to listen to
- PORT : port to listen to

A sample config.toml can be found in this folder
