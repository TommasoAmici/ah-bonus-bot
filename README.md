# AH Bonus Bot

This repo contains the source code for a Telegram bot that allows users to track products
on [ah.nl](https://www.ah.nl) and get notified when they go on sale.

## Usage

There are two components to this bot: the Telegram bot itself and a notification service.
The bot should always run to accept user input and to store the tracked products.

The notification service can be run on a schedule to check for price changes and send out
notifications if any tracked product is on sale. Since AH's discounts are valid for a week,
it is sufficient to run the notification service once a week on Monday.

Both binaries are built from the [`telegram_bot`](./crates/telegram_bot/) crate. After
building, the `bot` and `notify` binaries can be found in the `target/release` directory.

You can build the binaries with the following command:

```sh
cargo build --release
```

## Development

The [Makefile](./Makefile) contains targets to run both binaries locally, `make run` and
`make notify`. A a `.env` file with the following environment variables is required:

```sh
DATABASE_URL=sqlite:ah_bonus.db
TELOXIDE_TOKEN=XXX
```

You can request a bot token from the [BotFather](https://t.me/botfather).

### AH API

The reverse engineered AH API is documented in the [`ah_api`](./crates/ah_api/) crate.
