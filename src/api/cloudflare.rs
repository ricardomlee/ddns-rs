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
        let r = get_record(&self.name)?;
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
            r#type: String::from("A"),
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
            r#type: String::from("A"),
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
}

fn get_record(name: &String) -> Result<Record, Box<dyn error::Error>> {
    let zone_id = env::var("CF_ZONE")?;
    let token = env::var("CF_TOKEN")?;
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type=A&name={}",
        zone_id, name
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
    let content = match resp["result"][0]["content"].as_str() {
        Some(text) => text,
        None => return Ok(Record::new(None, name.to_string(), None)),
    };
    let id = match resp["result"][0]["id"].as_str() {
        Some(text) => text,
        None => return Ok(Record::new(None, name.to_string(), None)),
    };
    println!("got record for {}, ip: {}", name, content);
    let ip: IpAddr = content.parse()?;
    let record = Record::new(Some(ip), name.to_string(), Some(id.to_string()));
    Ok(record)
}
