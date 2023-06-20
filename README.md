# Multiplex

[![master](https://github.com/jpnws/multiplex/actions/workflows/master.yml/badge.svg)](https://github.com/jpnws/multiplex/actions/workflows/master.yml) [![audit](https://github.com/jpnws/multiplex/actions/workflows/audit.yml/badge.svg)](https://github.com/jpnws/multiplex/actions/workflows/audit.yml)

# Reference

For running tests without capturing debu std prinouts.

`cargo test -- --nocapture`

Execute following command for tracing:

`$env:RUST_LOG="trace"`

For Rust backtrace:

`$env:RUST_BACKTRACE=1`

For `tracing` crate logging with pretty print.
`cargo install bunyanz` first, and then:

`cargo watch -x check -x test -x run | bunyan`

For watching, checking, and running tests in realtime:

`cargo watch -x check -x test -x run`

Install bunyan log prettier.

`cargo install bunyan`

Running Docker.

`docker build --tag multiplex --file Dockerfile .`

`docker run -p 8000:8000 multiplex | bunyan`

## Do this whenever modifying spec.yaml.

1. Retrieve app ID with command: `doctl apps list --format ID`
2. Set the APP_ID env: `$env:APP_ID="<APP-ID>"`.
3. Update DigitalOcean's app with the updated spec:
    - `doctl apps update $env:APP_ID --spec spec.yaml`

## Docker notes

To pull a docker image for postgresql v15.3.

`docker pull postgres:15.3` or you can just run `docker run` command below.

To run the postgres docker image with dev config (for newsletter DB).

`docker run -d -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=newsletter -p 8090:5432 postgres:15.3`

- `-d` flag means to detach the docker run process from the shell and run in the background.
- `-e` sets up the environment variables for the docker container.
- `-p` is the port mapping from local->docker.
- `postgres` in `postgres:15.3` is the name of the docker image and `15.3` is the image's tag.
