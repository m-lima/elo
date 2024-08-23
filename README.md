# EloPong

[![Github](https://github.com/m-lima/elo/workflows/build/badge.svg)](https://github.com/m-lima/elo/actions?workflow=build)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

An elo scoring system for table tennis

## Build

### Backend

```
$ cd back
$ cargo build --release
```

### Frontend

```
$ cd front
$ yarn
$ yarn build
```

## Running locally

### Mock data

```
$ cd back
$ cargo r --features local -- -p 3333 -vvv -d <DB> -i <AMOUNT>
```

> `DB`: Path to the file to initialize and use as a Sqlite database
>
> `AMOUNT`: Number of games, on average, each player will register

### Launch backend

```
$ cd back
$ cargo r --features local -- -p 3333 -vvv -d <DB>
```

> `DB`: Path to the database initialized in the last step

> [!NOTE]
>
> When launching with `--features local`, the user will default to **test@email.com** unless overridden by the `X-USER` header

### Launch frontend

```
$ cd front
$ yarn dev --host
```

> [!NOTE]
>
> The `--host` parameter allows other devices in the network to access the app. Useful for testing on mobile devices

## Running in production

### Backend

If SMTP parameters are provided, **PongElo** will use them to send out emails, such as invitations and other notifications.

These parameters can be passed in the command line, or through the environment variables:

- `ELO_LINK`: Link to use in emails that point to where **PongElo** is deployed
  - E.g.: `https://pongelo.com`
- `ELO_FROM`: Canonical mailbox to send the emails from
  - E.g.: `PongElo <noreply-pongelo@email.com>`
- `ELO_SMTP`: Address to the SMTP server
  - E.g.: `smtp://smtp-relay.gmail.com:587?tls=required`

There is a [Dockerfile](./back/Dockerfile) provided that will run the backend on port 80. Additionally there is a [build script](./back/build.sh) that will generate the image with the above SMTP parameters and generate _systemd_ unit files

### Frontend

Neither the frontend nor the backend projects serve webpages. That means that, after [building](#frontend) the frontend, there needs to be a server to host the files and provide the header injection for authetication.

The recommended server is [nginx](https://nginx.org/) with SSL termination and an [OpenIDC](https://www.openidc.com/) layer to inject the `X-USER` header. But other servers can be used such as [caddy](https://caddyserver.com/)
