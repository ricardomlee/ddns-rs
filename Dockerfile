FROM rust:alpine3.16

ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /app

COPY . .

RUN cargo build --release \ 
    && strip -s target/release/ddns-rs

FROM alpine:latest

RUN set -eux && sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk add --no-cache openssl-dev

WORKDIR /app

COPY --from=0 /app/target/release/ddns-rs .

CMD ["./ddns-rs"]
