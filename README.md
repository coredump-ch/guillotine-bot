# guillotine

[![Build status](https://circleci.com/gh/coredump-ch/guillotine-bot.svg?style=shield&circle-token=:circle-token)](https://circleci.com/gh/coredump-ch/guillotine-bot)
[![Docker Image](https://img.shields.io/badge/docker%20image-coredump%2Fguillotine--bot-yellow)](https://hub.docker.com/r/coredump/guillotine-bot)

Telegram bot that "cuts off" 13:37 time at 13:38.

## Env vars

- `TELEGRAM_BOT_TOKEN`: The bot API token
- `TELEGRAM_GROUP_ID`: The group ID to target

## Logging

Set `RUST_LOG=guillotine=debug` (or `info` or `trace`).

## Docker Image

To build the docker image based on the current codebase:

    $ docker build -t coredump/guillotine-bot:latest .

Then launch a new container from the image:

    $ docker run -d --name guillotine-bot \
        -e TELEGRAM_BOT_TOKEN=... -e TELEGRAM_GROUP_ID=... \
        coredump/guillotine-bot

To stop it again:

    $ docker stop guillotine-bot

The docker image at https://hub.docker.com/r/coredump/guillotine-bot will be
automatically rebuilt on every push to master.

## License

Licensed under the WTFPL:

```
            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004

 Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

 Everyone is permitted to copy and distribute verbatim or modified
 copies of this license document, and changing it is allowed as long
 as the name is changed.

            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. You just DO WHAT THE FUCK YOU WANT TO.
```

<!-- Badges -->
[circle-ci]: https://circleci.com/gh/coredump-ch/guillotine-bot/tree/master
[circle-ci-badge]: https://circleci.com/gh/coredump-ch/guillotine-bot/tree/master.svg?style=shield
