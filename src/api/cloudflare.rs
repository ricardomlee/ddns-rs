use serde::Serialize;
use std::net::IpAddr;
use std::{env, error};

/* do not cache any api token info */
pub struct Record {
    ip: Option<IpAddr>,
    name: String,
    id: Option<String>,
}

#[derive(Serialize)]
struct Body {
    r#type: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
}

impl Record {
    pub fn new(ip: Option<IpAddr>, name: String, id: Option<String>) -> Record {
        Record {
            ip: ip,
            name: name,
            id: id,
        }
    }
    pub fn run_checker(&mut self) -> Result<(), Box<dyn error::Error>> {
        println!("🚛 cf ddns task: {} {}", self.name, self.ip.unwrap());
        let r = self.get_record()?;
        if r.ip == None {
            println!("no remote record, create record...");
            self.create()?;
            return Ok(());
        }
        self.id = r.id;
        if r.ip != self.ip {
            println!("ip changed from {} to {}", r.ip.unwrap(), self.ip.unwrap());
            println!("updating remote record...");
            self.update()?;
        } else {
            println!("ip not changed, no need to update remote record");
        }
        Ok(())
    }

    fn update(&self) -> Result<(), Box<dyn error::Error>> {
        let zone_id = env::var("CF_ZONE")?;
        let token = env::var("CF_TOKEN")?;
        let id = self.id.as_ref().unwrap();
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, id
        );
        let body = Body {
            r#type: get_record_type(&self.ip.unwrap()),
            name: self.name.clone(),
            content: self.ip.unwrap().to_string(),
            ttl: 1,
            proxied: false,
        };
        let client = reqwest::blocking::Client::new();
        let resp: serde_json::Value = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?
            .json()?;
        if resp["success"] == false {
            dbg!(&resp);
            println!("{}", resp["errors"][0]["message"].to_string());
            return Err("failed to update record".into());
        };
        println!("successfully updated record");
        Ok(())
    }

    fn create(&self) -> Result<(), Box<dyn error::Error>> {
        let zone_id = env::var("CF_ZONE")?;
        let token = env::var("CF_TOKEN")?;
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/",
            zone_id
        );
        let body = Body {
            r#type: get_record_type(&self.ip.unwrap()),
            name: self.name.clone(),
            content: self.ip.unwrap().to_string(),
            ttl: 1,
            proxied: false,
        };
        let client = reqwest::blocking::Client::new();
        let resp: serde_json::Value = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?
            .json()?;
        if resp["success"] == false {
            dbg!(&resp);
            println!("{}", resp["errors"][0]["message"].to_string());
            return Err("failed to create record".into());
        };
        println!("successfully created record");
        Ok(())
    }

    fn get_record(&self) -> Result<Record, Box<dyn error::Error>> {
        let zone_id = env::var("CF_ZONE")?;
        let token = env::var("CF_TOKEN")?;
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type={}&name={}",
            zone_id,
            get_record_type(&self.ip.unwrap()),
            self.name
        );
        let client = reqwest::blocking::Client::new();
        let resp: serde_json::Value = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .send()?
            .json()?;
        if resp["success"] == false {
            dbg!(&resp);
            println!("{}", resp["errors"][0]["message"].to_string());
            return Err("failed to query record".into());
        };
        let ip = match resp["result"][0]["content"].as_str() {
            Some(c) => Some(c.parse()?),
            None => None,
        };
        let id = match resp["result"][0]["id"].as_str() {
            Some(i) => Some(i.to_string()),
            None => None,
        };
        Ok(Record::new(ip, self.name.clone(), id))
    }
}

fn get_record_type(ip: &IpAddr) -> String {
    if ip.is_ipv4() {
        String::from("A")
    } else if ip.is_ipv6() {
        String::from("AAAA")
    } else {
        panic!("invalid record type!")
    }
}
