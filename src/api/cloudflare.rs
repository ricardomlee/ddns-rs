use std::net::IpAddr;

pub struct Record {
    ip: IpAddr,
    name: String,
}

impl Record {
    pub fn new(ip: IpAddr, name: String) -> Record {
        Record {
            ip: ip,
            name: name,
        }
    }
}

pub fn create_record(r: &Record) -> Result<String, String> {
    println!("create record for {}", r.name);
    println!("ip address is {}", r.ip);
    Ok("create success".to_string())
}

pub fn update_record(r: &Record) {
    println!("update record for {}", r.name);
    println!("ip address is {}", r.ip);
}

pub fn search_record(name: String) {
    println!("search record for {}", name);
}