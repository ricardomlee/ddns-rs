use local_ip_address::list_afinet_netifas;
use std::collections::HashMap;
use std::error;
use std::net::IpAddr;
use std::str::FromStr;

pub fn get_ip_from_system(if_name: &str, ip_type: &str) -> Result<IpAddr, Box<dyn error::Error>> {
    let network_interfaces = list_afinet_netifas().unwrap();
    match ip_type {
        "ipv4" => {
            let mut iter = network_interfaces
                .iter()
                .filter(|x| x.0 == if_name && x.1.is_ipv4());
            if let Some(inf) = iter.next() {
                return Ok(inf.1);
            } else {
                return Err("ip not found".into());
            }
        }
        "ipv6" => {
            let mut iter = network_interfaces
                .iter()
                .filter(|x| x.0 == if_name && x.1.is_ipv6());
            if let Some(inf) = iter.next() {
                return Ok(inf.1);
            } else {
                return Err("ip not found".into());
            }
        }
        _ => return Err("invalid ip type".into()),
    }
}

pub fn get_ip_from_net(ip_type: &str) -> Result<IpAddr, Box<dyn error::Error>> {
    match ip_type {
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
