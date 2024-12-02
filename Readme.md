# RansomFeed multibot

This rust bot runs multiple social network bots posting new entries from RansomFeed's RSS.

Currently, the following social networks are supported:

- bsky
- Telegram

More to come.

## Running

This bot is designed to run in docker and takes most of the arguments from the environment variables.
Some configuration parameter can be defined in a special config file.

### Mandatory env variables

The following environment variables are mandatory:

#### Blue Sky bot configuration

- `BSKY_USER` and `BSKY_PASS`: the username (email address) and app key of the account on bsky to which the bot will connect

#### Telegram bot configuration

- `TG_TOKEN`: the Telegram token used to access the bot

### Optional (but useful) env variables

- `DATADIR` is the prefix that will be attached to any persistent file, to allow them to be written from the
docker image. It must be writeable.
- `BSKY_DB` is the name of the bsky database, if you want to change the default name and location. By default,
the database will be named `bsky-bot.db` and will be located in `DATADIR` if present. If `BSKY_DB` is specified,
`DATADIR` will not be prepended.
- `BSKY_MAX_POST`: Set the maximum number of entries to post, to avoid flooding the server. This is useful when
starting the bot with an empty db.

## Building the docker image

The Dockerfile is a multistage recipe that can be used to build the final image:

Build it as usual:

```
$ docker build -t rfeed-bot-image .
```

**NOTE**: This is a multistage build. After the first build, cache must be invalidated.
Use this command

```
$ docker build -t rfeed-bot-image --no-cache-filter build-app,final .
```

Create a docker volume for data, for example named `rfeed-bot-data`

```
$ docker volume create rfeed-bot-data
```

## Running on docker

The following command line can be used to run the bot in a docker container:

```
$ docker run --rm --name rfeedbot \ 
    -e BSKY_USER=your-bsky@account \ 
    -e BSKY_PASS='your-bsky-pass-here' \
    -e DATADIR=\data \
    -v rfeed-bot-data:/data \
    rfeed-bot-image
```

