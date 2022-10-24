# ddns-rs

> ready for use with one cloudflare A/AAAA record 🥰

A ddns client written in Rust.


## Features
 - get public ip
 - cloudflare (A or AAAA record)
 - toml config
    - name: String
    - interval: u64
    - ip_type: String
    - interface(optional): String


## Install

```shell
cargo install --path .
# cargo install ddns-rs
```


## Usage

copy `config/ddns.toml` to `/tmp/ddns.toml` and update

```shell
export CF_ZONE=<your zone id>
export CF_TOKEN=<your api token>
ddns-rs
```


## TODO List
 - add dns vendors
    - aliddns
    - dnspod


## License

[MIT © ricardomlee](./LICENSE)
