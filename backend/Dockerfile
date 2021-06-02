#!/bin/bash
FROM rust:1.51.0  AS builder



WORKDIR /code
COPY . /code

ENV DATABASE_URL=postgres://postgres:123456@localhost:5433/postgres

RUN cargo build 

FROM debian:buster-slim

EXPOSE 8000

COPY --from=builder /code/target/release/react-rust-postgres /react-rust-postgres

CMD [ "/react-rust-postgres" ]
