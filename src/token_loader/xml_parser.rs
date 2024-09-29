use std::collections::HashMap;
use regex::Regex;

pub struct Node {
    children: Vec<Node>,
    node_type: String,
    node_data: Option<String>,
    attributes: HashMap<String, String>,
}

impl Node {
    pub fn new() -> Node {
        return Node {
            children: Vec::new(),
            node_type: String::with_capacity(16),
            node_data: None,
            attributes: HashMap::new(),
        };
    }
}

pub fn parse_dom(mut text: &str) {
    // quickly matching the version and comment from the xml since we don't care about those
    // if an issue occurs here, we don't care, but we should tell the user that they're being an idiot.
    match xml_version(text) {
        Some(txt) => text = txt,
        None => println!("Warning: No XML version specifier provided in tokensheet."),
    }
    match comment(text) {
        Some(txt) => text = txt,
        None => {}, // idk if this is how I am supposed to make None not do anything but it should work
    }

}



fn xml_version(file_slice: &str) -> Option<&str> {
    let version: Regex = Regex::new(r#"\A\w*<\?xml version="[0-9]\.[0-9]" encoding="UTF-8"\?>\n?"#).unwrap();
    match version.find(file_slice) {
        Some(result) => return Some(&file_slice[result.end()..]),
        None => return None,
    };
}

fn comment(file_slice: &str) -> Option<&str> {
    let comment: Regex = Regex::new(r"\A\w*<!--(?:.|\n|\r\n)*-->?").unwrap();
    match comment.find(file_slice) {
        Some(result) => return Some(&file_slice[result.end()..]),
        None => return None,
    }
}