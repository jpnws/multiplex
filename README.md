# Multiplex

[![master](https://github.com/jpnws/multiplex/actions/workflows/master.yml/badge.svg)](https://github.com/jpnws/multiplex/actions/workflows/master.yml) [![audit](https://github.com/jpnws/multiplex/actions/workflows/audit.yml/badge.svg)](https://github.com/jpnws/multiplex/actions/workflows/audit.yml)

# Reference

Execute following command for tracing:

`> $env:RUST_LOG="trace"`

For Rust backtrace:

`> $env:RUST_BACKTRACE=1`

For `tracing` crate logging with pretty print.
`cargo install bunyanz` first, and then:

`> cargo watch -x check -x test -x run | bunyan`

For watching, checking, and running tests in realtime:

`> cargo watch -x check -x test -x run`

Install bunyan log prettier.

`> cargo install bunyan`

Running Docker.

`> docker build --tag multiplex --file Dockerfile .`

`> docker run -p 8000:8000 multiplex | bunyan`

## Do this whenever modifying spec.yaml.

1. Retrieve app ID with command: `> doctl apps list --format ID`
2. Set the APP_ID env: `> $env:APP_ID="<APP-ID>"`.
3. Update DigitalOcean's app with the updated spec:
    - `> doctl apps update $env:APP_ID --spec spec.yaml`
