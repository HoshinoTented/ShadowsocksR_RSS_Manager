use std::collections::HashMap;
use super::config;

pub struct Command {
    pub command_name: Vec<&'static str>,
    pub action: Box<dyn FnOnce(&Vec<String>, &HashMap<String, String>) -> ()>,
    pub document: &'static str,
}

pub fn rss_file_from_config(config: &HashMap<String, String>) -> String {
    config.get("rssFile").expect("need property 'rssFile'").to_string()
}

pub fn url_from_config(config: &HashMap<String, String>) -> String {
    config.get("url").expect("need property 'url'").to_string()
}

pub fn commands() -> Vec<Command> {
    vec![
        Command {
            command_name: vec!["update"],
            action: Box::new(|args, config| {
                let rss_file = rss_file_from_config(config);
                let url = url_from_config(config);

                println!("Updating...");
                super::config::update_rss(&rss_file, &url).unwrap();
                println!("Done.");
            }),
            document: r#"
Usage: ssrmanager update [OPTIONS]
    Update rss from given rss link.
            "#.trim(),
        },
        Command {
            command_name: vec!["show", "info"],
            action: Box::new(|arguments, config| {
                let rss_file = rss_file_from_config(config);

                let index: usize = arguments.get(0).expect("show need a value 'index'").parse().expect("index must be a number");
                let mut nodes = config::nodes_from_file(&rss_file);
                let node = nodes.get_mut(index).expect(&format!("node not found by index: {}", index));

                node.password = "<HIDDEN>".to_string();

                println!("{:?}", node);
            }),
            document: r#"
Usage: ssrmanager <show|info> <index> [OPTIONS]
    Show node information by given index.
        "#.trim(),
        },
        Command {
            command_name: vec!["ls", "list"],
            action: Box::new(|args, config| {
                let rss_file = rss_file_from_config(config);

                for (i, node) in super::config::nodes_from_file(&rss_file).into_iter().enumerate() {
                    println!("{}: {}", i, node.name);
                }
            }),
            document: r#"
Usage: ssrmanager <ls|list> [OPTIONS]
    Show nodes list from rss file.
        "#.trim(),
        }
    ]
}