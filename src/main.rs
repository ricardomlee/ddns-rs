mod api;
use api::{cloudflare, ip};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!(
        "Hello, ipv4: {}",
        ip::get_ip_from_system(ip::IpFrom::Shell("ls".to_string()))
    );
    println!(
        "Hello, ipv4: {}",
        ip::get_ip_from_system(ip::IpFrom::Manual(192, 168, 123, 106))
    );

    println!("Hello, ipv6: {}", ip::get_ip_from_net());
    let r = cloudflare::Record::new(
        ip::get_ip_from_system(ip::IpFrom::Manual(239, 0, 0, 1)),
        String::from("liming.mc"),
    );
    let ret = cloudflare::create_record(&r);
    match ret {
        Ok(s) => println!("{}", s),
        Err(s) => println!("{}", s),
    }
    cloudflare::update_record(&r);
    cloudflare::search_record("liming.ml".to_string());
    Ok(())
}


