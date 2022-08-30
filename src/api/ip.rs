use std::collections::HashMap;
use std::error;
use std::net::{IpAddr};
use std::str::FromStr;
use local_ip_address::list_afinet_netifas;

pub fn get_ip_from_system(if_name: &str) -> Result<IpAddr, Box<dyn error::Error>> {
    let network_interfaces = list_afinet_netifas().unwrap();

    for(name, ip) in network_interfaces.iter() {
        println!("{}:\t{:?}", name, ip);
        if name == if_name {
            return Ok(*ip);
        }
    }

    return Err("interface not found".into());
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
