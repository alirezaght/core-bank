FROM rust:1.58.1-slim-buster



RUN apt-get update -qq && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres


WORKDIR /app


COPY . .


RUN cargo build --release


ENTRYPOINT ["diesel migration run && ./target/release/core-bank"]
