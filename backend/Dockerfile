#!/bin/ash
FROM rust AS builder

RUN apt-get update \
  && apt-get install -y libpq-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /code
COPY . /code

ENV DATABASE_URL=postgres://postgres:123456@localhost:5433/postgres

RUN cargo build 

FROM debian:buster-slim

COPY --from=builder /code/target/release/react-rust-postgres /react-rust-postgres

CMD [ "/react-rust-postgres" ]
