use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;

pub enum IpFrom {
    Shell(String),
    Manual(u8, u8, u8, u8),
}

pub fn get_ip_from_system(from: IpFrom) -> IpAddr {
    match from {
        IpFrom::Manual(b, c, d, e) => {
            println!("new value {}", b);
            return IpAddr::V4(Ipv4Addr::new(b, c, d, e));
        }
        IpFrom::Shell(s) => {
            println!("cmd: {}", s);
            let output = Command::new(s).output().expect("run error");
            let out = String::from_utf8(output.stdout).unwrap();
            println!("{}", output.status);
            println!("{}", out);
        }
    }

    IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))
}

pub fn get_ip_from_net() -> IpAddr {
    let b: u16 = 0xFF;
    IpAddr::V6(Ipv6Addr::new(b, b, b, b, b, b, b, b))
}