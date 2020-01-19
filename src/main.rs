mod ssr;
mod config;

use base64;
use std::fs;
use std::env::args;
use regex::Regex;
use crate::config::update_rss;

#[derive(Debug)]
pub enum Argument {
    Option(String),
    Value(String),
}

pub fn expect_value(this: Option<Argument>, name: &'static str) -> String {
    match this {
        Some(v) => match v {
            Argument::Option(_) => panic!("{} expect a value", name),
            Argument::Value(v) => v
        },

        None => panic!("{} expect a value", name)
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

    if help != None || command == None {
        println!("{}", r#"
Usage: ssr_manager_rs [OPTIONS]
    h | help              => Print help document
    p | propertyFile      => Specified config file path (default: "./config.properties")
    u | url               => Specified rss url
    r | rssFile           => Specified rss file path (default: "./rss.txt")
    t | template          => Specified template file path (default: "./template.json")

    update                => Update rss (need field url)
    ls   | list           => Show node list from rss file
    show | info <index>   => Show node info from rss file by given index
        "#.trim())
    } else {
        let property_file = property_file.unwrap_or("config.properties".to_string());
        let config = config::get_config(&property_file);
        let url = url.unwrap_or_else(|| config.get("url").expect("url not found").to_string());
        let rss_file = rss_file.unwrap_or_else(|| config.get("rssFile").unwrap_or(&"rss.txt".to_string()).to_string());
        let command = command.unwrap();

        match command.as_str() {
            "update" => {
                println!("Updating...");
                update_rss(&rss_file, &url).unwrap();
                println!("Done.");
            }

            "ls" | "list" => {
                for (i, node) in config::nodes_from_file(&rss_file).into_iter().enumerate() {
                    println!("{}: {}", i, node.name);
                }
            }

            "show" | "info" => {
                let index: usize = arguments.get(0).expect("show need a value 'index'").parse().expect("index must be a number");
                let mut nodes = config::nodes_from_file(&rss_file);
                let node = nodes.get_mut(index).expect(&format!("node not found by index: {}", index));

                node.password = "<HIDDEN>".to_string();

                println!("{:?}", node);
            }

            _ => {
                panic!("Unknown command: {}", command);
            }
        }
    }
}
