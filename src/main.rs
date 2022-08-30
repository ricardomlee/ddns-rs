mod api;
use api::{cloudflare, ip};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct Config {
    name: Option<String>,
    interval: Option<u64>,
    ip_type: Option<String>,
    interface: Option<String>,
}

#[derive(Deserialize)]
struct ConfVec {
    ddns_config: Option<Vec<Config>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = "/tmp/ddns.toml";
    let mut file = File::open(config_file)?;
    let mut str_val = String::new();
    file.read_to_string(&mut str_val)?;
    let config: ConfVec = toml::from_str(&str_val).unwrap();
    let mut name = String::new();
    let mut ip_type = String::from("ipv4");
    let mut interval: u64 = 300;
    let mut if_name = String::new();
    for x in config.ddns_config.unwrap() {
        name = x.name.unwrap();
        ip_type = x.ip_type.unwrap_or_else(|| ip_type);
        interval = x.interval.unwrap_or_else(|| interval);
        if_name = x.interface.unwrap_or_else(|| if_name);
    }
    let mut fail_cnt = 0;
    loop {
        let mut r =
            if !if_name.is_empty() { 
                cloudflare::Record::new(Some(ip::get_ip_from_system(&if_name)?), name.clone(), None)
            } else {
                cloudflare::Record::new(Some(ip::get_ip_from_net(&ip_type)?), name.clone(), None)
            };
        println!("-------------------------");
        match r.run_checker() {
            Ok(_) => fail_cnt = 0,
            Err(_) => fail_cnt += 1,
        };
        println!("-------------------------");
        if fail_cnt > 5 {
            break;
        }
        std::thread::sleep(std::time::Duration::new(interval, 0));
    }
    Ok(())
}
