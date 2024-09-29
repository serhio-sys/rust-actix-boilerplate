## Build
FROM rust:1.81.0 as build

ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /src/rust-app/
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \
    cargo build --release

## Deploy
FROM ubuntu:22.04 as runtime

## Copy compiled runnable file
COPY --from=build /src/rust-app/target/release/server /usr/local/bin/rust-app/server 

## Copy migrations
COPY --from=build /src/rust-app/internal/infra/database/migrations /usr/local/bin/rust-app/migrations

## Dependencies related to postgreSQL
RUN apt-get update && apt-get install -y libpq-dev
RUN apt-get update && apt-get install libc6

EXPOSE 8080

WORKDIR /usr/local/bin/rust-app/
CMD ["./server"]