mod ssr;
mod config;
mod command;

use base64;
use std::fs;
use std::env::args;
use regex::Regex;
use crate::config::update_rss;
use std::ops::Deref;

#[derive(Debug)]
pub enum Argument {
    Option(String),
    Value(String),
}

pub fn expect_value(this: Option<Argument>, name: &'static str) -> String {
    match this {
        Some(Argument::Value(v)) => v,
        _ => panic!("{} expect a value", name)
    }
}

fn main() {
    let mut args = args().map(|arg| {
        if arg.starts_with("-") {
            Argument::Option(arg.trim_start_matches("-").to_string())
        } else {
            Argument::Value(arg)
        }
    });

    let mut property_file: Option<String> = None;
    let mut rss_file: Option<String> = None;
    let mut url: Option<String> = None;
    let mut help: Option<()> = None;
    let mut values = Vec::new();

    args.next();

    while let Some(arg) = args.next() {
        match arg {
            Argument::Option(name) => {
                match name.as_str() {
                    "p" | "propertyFile" => {
                        property_file = Some(expect_value(args.next(), "propertyFile"))
                    }

                    "u" | "url" => {
                        url = Some(expect_value(args.next(), "url"))
                    }

                    "h" | "help" => {
                        help = Some(())
                    }

                    "r" | "rssFile" => {
                        rss_file = Some(expect_value(args.next(), "rssFile"))
                    }

                    _ => {
                        panic!("Unknown option: {}", name)
                    }
                }
            }

            Argument::Value(v) => {
                values.push(v);
            }
        }
    }

    let (command, arguments) = {
        let mut it = values.into_iter();
        (it.next(), it.collect::<Vec<String>>())
    };

    if command == None {
        println!("{}", r#"
Usage: ssr_manager_rs [Command] [OPTIONS]
    h | help              => Print help document
    p | propertyFile      => Specified config file path (default: "./config.properties")
    u | url               => Specified rss url
    r | rssFile           => Specified rss file path (default: "./rss.txt")
    t | template          => Specified template file path (default: "./template.json") [UNIMPLEMENTATION]

  Commands:
    update                => Update rss (need field url)
    ls   | list           => Show node list from rss file
    show | info <index>   => Show node info from rss file by given index
        "#.trim())
    } else {
        let property_file = property_file.unwrap_or("config.properties".to_string());
        let mut config = config::get_config(&property_file);
        if let Some(url) = url { config.insert("url".to_string(), url); }
        if let Some(rss_file) = rss_file { config.insert("rssFile".to_string(), rss_file); }
        if !config.contains_key("rssFile") { config.insert("rssFile".to_string(), "rss.txt".to_string()); }
        let command = command.unwrap();


        'outer: for cmd in command::commands().into_iter() {
            for &name in cmd.command_name.iter() {
                if name == command.as_str() {
                    if help == None {
                        (cmd.action)(&arguments, &config);
                    } else {
                        println!("{}", cmd.document);
                    }

                    return;
                }
            }
        }

        panic!("Unknown command: {}", command);
    }
}
