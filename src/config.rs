use super::ssr;
use std::fs;
use std::collections::HashMap;

pub fn update_rss(rss_file: &str, url: &str) -> Result<(), reqwest::Error> {
    let text = reqwest::blocking::get(url)?.text()?;
    std::fs::write(rss_file, text).expect("io error");

    Ok(())
}

pub fn nodes_from_file(file: &str) -> Vec<ssr::Node> {
    ssr::resolve_ssr_rss(
        &ssr::decode(&String::from_utf8(fs::read(file).expect("io error")).unwrap())
    ).expect("error when parsing rss")
}

pub fn get_config(path: &str) -> HashMap<String, String> {
    fn parse_line(line: &str) -> (String, String) {
        for (i, c) in line.chars().enumerate() {
            if c == '=' {
                return (line[0..i].to_string(), line[i + 1..].to_string());
            }
        }

        panic!("can not parse config file.")
    }

    let text = String::from_utf8(fs::read(path).expect("can not read text from file")).unwrap();
    let mut config = HashMap::new();

    for line in text.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("#")) {
        let (name, value) = parse_line(line);
        config.insert(name, value);
    }

    config
}