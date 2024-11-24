# Flagsmith Rust Project

## Tools used
- [Task](https://taskfile.dev)
- [Redis](https://redis.io/)
- [Docker](https://www.docker.com/)
- [Flagsmith](https://www.flagsmith.com/)
- [Rust](https://www.rust-lang.org/)
- [Rocket.rs](https://rocket.rs/guide/v0.5/getting-started/#getting-started)

## Setup Steps

### Local Setup
- Clone the Repo
- run `cargo install` to install deps
- create `.env` file to create the following environment variables in local
```shell
REDIS_URL=localhost:6379
PORT=8080
RATE_LIMIT=10
FLAGSMITH_ENVIRONMENT_KEY=ser.ZRd***********469
```
- run `task up` to bring up the local redis cluster
- run `task local` to open the local gin server

### Docker Setup

- Follow the above steps
- run `task db` to build the docker image
- run `task dr` to run the local docker image