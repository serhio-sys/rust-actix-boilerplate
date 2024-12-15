## Deploy
FROM ubuntu:22.04

## Copy compiled runnable file
COPY target/release/server /usr/local/bin/rust-app/server 

## Copy migrations
COPY internal/infra/database/migrations /usr/local/bin/rust-app/migrations

## Copy env file
COPY .docker/env/.app.env /usr/local/bin/rust-app/.env

## Dependencies related to postgreSQL
RUN apt-get update && apt-get install -y libpq-dev
RUN apt-get update && apt-get install libc6

EXPOSE 8080

WORKDIR /usr/local/bin/rust-app/

CMD ["./server"]