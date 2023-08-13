FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml /app/
COPY src /app/src

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/ddns-rs /app/

CMD ["./ddns-rs"]
