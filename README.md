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


## Install

```shell
cargo install --path .
# cargo install ddns-rs
```


## Usage

update and copy `config/ddns.toml` to `/tmp/ddns.toml`

```shell
export CF_ZONE=<your zone id>
export CF_TOKEN=<your api token>
ddns-rs
```


## TODO List

 - get ip
    - from interface
    - customizable api
 - multiple records
    - multi-threads
    - config iterator
    - token/id for each
 - async functions
 - add dns vendors
    - aliddns
    - dnspod
 - encrypted token/id


## License

[MIT © ricardomlee](./LICENSE)
