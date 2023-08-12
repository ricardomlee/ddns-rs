FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release \ 
    && strip -s target/release/ddns-rs

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/ddns-rs .

CMD ["./ddns-rs"]
