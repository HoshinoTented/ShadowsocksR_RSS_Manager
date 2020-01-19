use base64;
use regex::Regex;
use std::fmt::{Formatter, Error};
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, ResolveError>;

#[derive(Debug)]
pub struct ResolveError {
    message: String
}

impl ResolveError {
    pub fn new(msg: &str) -> ResolveError {
        ResolveError {
            message: String::from(msg)
        }
    }

    pub fn not_found(name: &str) -> ResolveError {
        ResolveError::new(&format!("can not found '{}' in link", name))
    }
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        f.write_str(&format!("ResolveError {{ message: {} }}", self.message))
    }
}

impl std::error::Error for ResolveError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

#[derive(Debug)]
pub struct Node {
    pub ip: String,
    pub port: u32,
    pub protocol: String,
    pub method: String,
    pub obfs: String,
    pub password: String,
    pub obfsparam: String,
    pub name: String,
    pub group: String,
}

pub fn decode(input: &str) -> String {
    let fix = input.len() % 4;

    let fixed = if fix != 0 {
        String::from(input) + &"=".repeat(4 - fix)
    } else {
        String::from(input)
    };

    String::from_utf8(
        base64::decode(&fixed.replace("-", "+").replace("_", "/"))
            .expect("decode base64 failed")
    ).expect("qaq")
}


pub fn resolve_ssr_raw(raw: &str) -> Result<Node> {
    fn check<T>(value: Option<T>, name: &'static str) -> Result<T> {
        value.ok_or(ResolveError::not_found(name))
    }

    let mut parts = raw.split(r"/?");
    let link = check(parts.next(), "link")?;
    let params = check(parts.next(), "params")?;
    let mut link_info = link.split(":").map(|v| v.to_string());
    let ip = check(link_info.next(), "ip")?;
    let port: u32 = check(link_info.next(), "port")?.parse().unwrap();
    let protocol = check(link_info.next(), "protocol")?;
    let method = check(link_info.next(), "method")?;
    let obfs = check(link_info.next(), "obfs")?;
    let password = decode(&check(link_info.next(), "password")?);
    let params = params.split("&");
    let params = {
        let mut ps = HashMap::new();

        for pair in params {
            let mut it = pair.split("=");
            let name = it.next().unwrap();
            let value = it.next().unwrap();

            ps.insert(name, decode(value));
        }

        ps
    };

    Ok(Node {
        ip,
        port,
        protocol,
        method,
        obfs,
        password,
        obfsparam: check(params.get("obfsparam"), "obfsparam")?.clone(),
        name: check(params.get("remarks"), "remarks")?.clone(),
        group: check(params.get("group"), "group")?.clone(),
    })
}

pub fn resolve_ssr_link(link: &str) -> Result<Node> {
    if link.starts_with("ssr://") {
        resolve_ssr_raw(&decode(&String::from(link)[6..]))
    } else {
        return Err(ResolveError::new("Link does not starts with 'ssr://'"));
    }
}

pub fn resolve_ssr_rss(raw: &str) -> Result<Vec<Node>> {
    raw.split("\n")
        .filter(|link| !link.is_empty())
        .map(|link| resolve_ssr_link(link))
        .collect()
}