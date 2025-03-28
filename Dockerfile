FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache \
    build-base \
    openssl-dev \
    pkgconfig

ENV OPENSSL_DIR=/usr

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/home-automation /app/

CMD ["./home-automation"]

