FROM rust:alpine3.16
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN set -eux && sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

COPY . .

RUN cargo build --release \ 
    && strip -s target/release/ddns-rs

FROM alpine:latest
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc

WORKDIR /app

COPY --from=0 /app/target/release/ddns-rs .

CMD ["./ddns-rs"]
