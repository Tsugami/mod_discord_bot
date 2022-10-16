# Mod Discord Bot

moderation bot that saves the voice connection history of the guild members

## How to run migrations

### Installation SQLX-CLI

`cargo install sqlx-cli --no-default-features --features rustls,postgres`

### Run migrations

`cargo sqlx migrate run`

### Create migration

`cargo sqlx migration add <name>`
