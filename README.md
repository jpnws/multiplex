# Multiplex

[![master](https://github.com/jpnws/multiplex/actions/workflows/master.yml/badge.svg)](https://github.com/jpnws/multiplex/actions/workflows/master.yml) [![audit](https://github.com/jpnws/multiplex/actions/workflows/audit.yml/badge.svg)](https://github.com/jpnws/multiplex/actions/workflows/audit.yml)

## Do this whenever modifying `spec.yaml`.

1. Retrieve app ID with command:

    `doctl apps list --format ID`

2. Set the APP_ID env:

    `$env:APP_ID="<APP-ID>"`

3. Update DigitalOcean's app with the updated spec:

    `doctl apps update $env:APP_ID --spec spec.yaml`

### Consequence of not syncing `spec.yaml` with DigitalOcean

- You may see an error that looks like:

```
2023-06-20T23:48:12.361939011Z [34m│[0m [36mINFO[0m[0403] RUN cargo build --release --bin multiplex
2023-06-20T23:48:13.674296186Z [34m│[0m [36mINFO[0m[0404] Cmd: /bin/sh
2023-06-20T23:48:13.674331182Z [34m│[0m [36mINFO[0m[0404] Args: [-c cargo build --release --bin multiplex]
2023-06-20T23:48:13.674335744Z [34m│[0m [36mINFO[0m[0404] Running: [/bin/sh -c cargo build --release --bin multiplex]
2023-06-20T23:48:14.535293674Z [34m│[0m    Compiling multiplex v0.1.0 (/app)
2023-06-20T23:48:15.012673167Z [34m│[0m error: failed to find data for query
2023-06-20T23:48:15.012710993Z [34m│[0m                INSERT INTO subscriptions (id, email, name, created_at, status)
2023-06-20T23:48:15.012732736Z [34m│[0m                VALUES ($1, $2, $3, $4, $5)
2023-06-20T23:48:15.012736422Z [34m│[0m
2023-06-20T23:48:15.012739258Z [34m│[0m    --> src/routes/subscriptions.rs:91:5
2023-06-20T23:48:15.012742746Z [34m│[0m     |
2023-06-20T23:48:15.012746126Z [34m│[0m 91  | /     sqlx::query!(
2023-06-20T23:48:15.012748870Z [34m│[0m 92  | |         r#"
2023-06-20T23:48:15.012751799Z [34m│[0m 93  | |         INSERT INTO subscriptions (id, email, name, created_at, status)
2023-06-20T23:48:15.012758810Z [34m│[0m 94  | |         VALUES ($1, $2, $3, $4, $5)
2023-06-20T23:48:15.012761857Z [34m│[0m ...   |
2023-06-20T23:48:15.012764724Z [34m│[0m 100 | |         "pending_confirmation"
2023-06-20T23:48:15.012767482Z [34m│[0m 101 | |     )
2023-06-20T23:48:15.012770251Z [34m│[0m     | |_____^
2023-06-20T23:48:15.012773064Z [34m│[0m     |
2023-06-20T23:48:15.013224490Z [34m│[0m     = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
2023-06-20T23:48:15.013526986Z [34m│[0m
2023-06-20T23:48:15.090388933Z [34m│[0m warning: unused import: `chrono::Utc`
2023-06-20T23:48:15.090407269Z [34m│[0m  --> src/routes/subscriptions.rs:2:5
2023-06-20T23:48:15.090411326Z [34m│[0m   |
2023-06-20T23:48:15.090414710Z [34m│[0m 2 | use chrono::Utc;
2023-06-20T23:48:15.090418239Z [34m│[0m   |     ^^^^^^^^^^^
2023-06-20T23:48:15.090421084Z [34m│[0m   |
2023-06-20T23:48:15.090423884Z [34m│[0m   = note: `#[warn(unused_imports)]` on by default
2023-06-20T23:48:15.090426772Z [34m│[0m
2023-06-20T23:48:15.090429634Z [34m│[0m warning: unused import: `uuid::Uuid`
2023-06-20T23:48:15.090432418Z [34m│[0m  --> src/routes/subscriptions.rs:4:5
2023-06-20T23:48:15.090435238Z [34m│[0m   |
2023-06-20T23:48:15.090438030Z [34m│[0m 4 | use uuid::Uuid;
2023-06-20T23:48:15.090440834Z [34m│[0m   |     ^^^^^^^^^^
2023-06-20T23:48:15.090443640Z [34m│[0m
2023-06-20T23:48:15.602861181Z [34m│[0m warning: `multiplex` (lib) generated 2 warnings
2023-06-20T23:48:15.602887778Z [34m│[0m error: could not compile `multiplex` (lib) due to previous error; 2 warnings emitted
2023-06-20T23:48:15.614469236Z [34m│[0m error building image: error building stage: failed to execute command: waiting for process to exit: exit status 101
2023-06-20T23:48:15.695522300Z [34m│[0m
2023-06-20T23:48:15.695877711Z [34m│[0m command exited with code 101
2023-06-20T23:48:15.696188071Z [34m│[0m
2023-06-20T23:48:16.029894861Z [34m│[0m [31m ✘ build failed[0m
```

## Do this whenever doing `sqlx` migration.

- Confirm that `sqlx` CLI installed on your system:

    `cargo install --version="~0.6" sqlx-cli --no-default-features --features rustls,postgres`

- Run sqlx prepare to create or update `sqlx-data.json`:

    `cargo sqlx prepare`

## Docker notes

- To pull a docker image for postgresql v15.3, execute:

    `docker pull postgres:15.3`

- To run the postgres docker image with dev config (for newsletter DB).

    `docker run -d -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=newsletter -p 8090:5432 postgres:15.3`

    - `-d` flag means to detach the docker run process from the shell and run in the background.
    - `-e` sets up the environment variables for the docker container.
    - `-p` is the port mapping from local->docker.
    - `postgres` in `postgres:15.3` is the name of the docker image and `15.3` is the image's tag.

## Reference

- For running tests without capturing debug std printouts.

    `cargo test -- --nocapture`

- Execute following command for tracing:

    `$env:RUST_LOG="trace"`

- For Rust backtrace:

    `$env:RUST_BACKTRACE=1`

- For `tracing` crate logging with pretty print.

    `cargo install bunyanz` first, and then:

    `cargo watch -x check -x test -x run | bunyan`

- For watching, checking, and running tests in realtime:

    `cargo watch -x check -x test -x run`

- Install bunyan log prettier.

    `cargo install bunyan`

- Running Docker.

    `docker build --tag multiplex --file Dockerfile .`

    `docker run -p 8000:8000 multiplex | bunyan`
