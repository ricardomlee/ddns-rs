use std::collections::HashMap;
use std::error;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::str::FromStr;

pub enum IpFrom {
    Shell(String),
    Manual(u8, u8, u8, u8),
}

// TODO
pub fn get_ip_from_system(from: IpFrom) -> IpAddr {
    match from {
        IpFrom::Manual(b, c, d, e) => IpAddr::V4(Ipv4Addr::new(b, c, d, e)),
        IpFrom::Shell(s) => {
            let output = Command::new("curl").arg(s).output().expect("run error");
            let out = String::from_utf8(output.stdout).unwrap();
            IpAddr::from_str(&out).unwrap()
        }
    }
}

pub fn get_ip_from_net(ip_type: &String) -> Result<IpAddr, Box<dyn error::Error>> {
    match ip_type as &str {
        "ipv4" => {
            let resp = reqwest::blocking::get("https://httpbin.org/ip")?
                .json::<HashMap<String, String>>()?;
            Ok(IpAddr::from_str(resp.get("origin").unwrap())?)
        }
        "ipv6" => {
            let resp = reqwest::blocking::get("https://api6.ipify.org/?format=json")?
                .json::<HashMap<String, String>>()?;
            Ok(IpAddr::from_str(resp.get("ip").unwrap())?)
        }
        _ => Err("invalid ip type".into()),
    }
}
