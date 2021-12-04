# ddns-rs

> ready for use with one cloudflare A record 🥰

A ddns client written in Rust.


## Install

```shell
cargo install --path .
# cargo install ddns-rs
```

## Usage

update and copy `config/ddns.toml` to `/tmp/ddns.toml`

```shell
CF_ZONE=<your zone id> CF_TOKEN=<your api token> ddns-rs
```

## Features
 - get public ip
 - cloudflare (A record)
 - toml config
    - name
    - interval


## TODO List

 - get_ip
    - from interface
 - multiple records
 - async functions
 - more config
 - more dns vendor


## Contributing

PRs accepted.

## License

[MIT © ricardomlee](./LICENSE)
