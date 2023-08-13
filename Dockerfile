FROM rust:alpine3.16

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN set -eux && sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

COPY . .

RUN cargo build --release \ 
    && strip -s target/release/ddns-rs

FROM alpine:latest

RUN set -eux && sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk add --no-cache openssl

WORKDIR /app

COPY --from=0 /app/target/release/ddns-rs .

CMD ["./ddns-rs"]
