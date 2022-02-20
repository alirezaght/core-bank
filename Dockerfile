FROM rust:1.58.1-slim-buster as builder

WORKDIR /app


RUN apt-get update -qq && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

ADD . .

RUN cargo build --release --bin core-bank


FROM debian:stable-20220125-slim as runtime

RUN apt-get update -qq && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/core-bank /usr/local/bin/

ENTRYPOINT ["./usr/local/bin/core-bank"]
