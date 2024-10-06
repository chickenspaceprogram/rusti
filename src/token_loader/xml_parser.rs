use std::{error::Error, fmt, sync::LazyLock};
use regex::Regex;

#[derive(Debug)]
pub struct XMLParsingError;

impl Error for XMLParsingError {}

impl fmt::Display for XMLParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "The XML could not be parsed. Maybe you forgot an ending tag?")
    }
}

/// A node of the DOM for the parsed XML
#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: Option<String>,
    pub attributes: Vec<Attribute>,
    pub data: Option<String>,
}

#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

// global variables aren't great but they were neater than passing the same regexes everywhere or compiling them repeatedly
static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\A<([a-zA-Z_:][a-zA-Z_:\-.0-9]*)(?: [a-zA-Z_:][a-zA-Z_:\-.0-9]*=['"].+?['"])*>"#).unwrap());
static DATA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\A(.*?)</([a-zA-Z_:][a-zA-Z_:\-.0-9]*)>"#).unwrap());
static WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\s*").unwrap());


enum State {
    Key,
    Value,
}

impl Node {
    /// Creates a new Node with `nodetype` as its type.
    /// This type of node does not have data, but it can 
    pub fn new_structure_node(nodetype: String) -> Node {
        return Node {
            children: Vec::new(),
            node_type: Some(nodetype),
            attributes: Vec::new(),
            data: None,
        }
    }
    pub fn new_data_node(data: &String) -> Node {
        return Node {
            children: Vec::new(),
            node_type: None,
            attributes: Vec::new(),
            data: Some(data.clone()),
        }
    }
    
    /// Sets any attributes of the node given the tag associated with it
    /// Pass the entire matched tag into this function, <brackets> and all.
    fn set_attributes(&mut self, tag: String) {

        let mut key: String = String::new();
        let mut val: String = String::new();
        let mut tag_iter = tag.chars().peekable();

        match tag.chars().position(|current_char| current_char == ' ') {
            Some(start_position) => {
                tag_iter.nth(start_position);
            },
            None => {
                return;
            },
        }
        let mut state = State::Key;
        
        // this much nesting hurts my soul
        while let Some(current_char) = tag_iter.next() {
            match state {
                State::Key => {
                    match current_char {
                        ' ' => {}, // dont really care about spaces
                        '=' => {
                            state = State::Value;
                            tag_iter.next(); // advancing the iterator so that the value state doesn't have to deal with opening/closing quotes
                        }
                        _ => key.push(current_char),
                    }
                }
                State::Value => {
                    match current_char {
                        '>' => {
                            self.attributes.push(
                                Attribute {
                                    key: key.clone(),
                                    value: val.clone(),
                                }
                            );
                            return;
                        }
                        '"' | '\'' => {
                            self.attributes.push(
                                Attribute {
                                    key: key.clone(),
                                    value: val.clone(),
                                }
                            );
                            key.clear();
                            val.clear();
                            state = State::Key
                        }
                        _ => val.push(current_char),
                    }
                }
            }
        }
    }
}

/// A function to parse an XML file into a DOM.
/// 
/// This isn't a general XML parser, it's only intended to parse the tokensheet specifically.
/// It could probably be turned into a more general, proper XML parser if I cared and had the time, but this works.
pub fn parse_xml(mut text: &str) -> Result<Node, XMLParsingError> {

    // quickly matching the version and comment from the XML since we don't care about those
    // if an issue occurs here, we don't care, but we should tell the user that they're being an idiot.
    match xml_version(text) {
        Some(txt) => text = txt,
        None => panic!("Warning: Either no XML version specifier was provided in tokensheet or the specifier was incorrectly formatted."),
    }
    match comment(text) {
        Some(txt) => text = txt,
        None => {}, // idk if this is how I am supposed to make None not do anything but it should work
    }
    let mut main_node = Node::new_structure_node("main".to_string());

    match make_node(Ok(text), &mut main_node) {
        Ok(..) => return Ok(main_node), // actually check for errors lol
        Err(XMLParsingError) => return Err(XMLParsingError),
    }
}


fn make_node<'a>(file_slice: Result<&'a str, XMLParsingError>, last_node: &mut Node) -> Result<Option<&'a str>, XMLParsingError>{
    // passing in references to all the regexes is cursed but I couldn't think of a better way
    let end_tag: String;
    let mut slice: &str; // slice itself is mutable, but the reference it holds isn't. so cursed. 
    slice = file_slice?;
    
    slice = grab_whitespace(slice);
    match TAG.captures(slice) {
        Some(result) => {
            let mut current_node: Node = Node::new_structure_node(result.get(1).unwrap().as_str().to_string()); // we can unwrap because this will always be in the match
            end_tag = format!("</{}>", current_node.node_type.clone().unwrap()); // unwrap is safe here, we just set the node type so we know it is not None
            slice = &slice[result.get(0).unwrap().len()..]; // advancing slice

            // grabbing all of the subtags we can. make_node will put those subtags into current_node.children so we don't have to worry about anything
            loop {
                match make_node(Ok(slice), &mut current_node)? { // if an error's been thrown we also want to throw that error
                    Some(new_slice) => {
                        slice = new_slice;
                    }
                    None => {
                        slice = grab_whitespace(slice);
                        break;
                    },
                }
            }
            // if the next tag isn't our end tag, the XML is not formatted correctly
            if slice[..end_tag.len()] != end_tag {
                return Err(XMLParsingError);
            } else {
                last_node.children.push(current_node);
                return Ok(Some(&slice[end_tag.len()..]))
            }

        }
        None => {
            let mut data = String::new();
            let mut current_char: char;
            let current_node: Node;
            let mut chars = slice.chars();
            loop {
                match chars.next() {
                    Some(this_char) => current_char = this_char.clone(),
                    None => break,
                }
                if current_char == '<' {
                    break;
                }
                data.push(current_char);
            }
            if !data.is_empty() {
                current_node = Node::new_data_node(&data);
                last_node.children.push(current_node);
                return Ok(Some(&slice[data.len()..]));
            } else {
                return Ok(None);
            }
        }
    }
}

fn grab_whitespace<'a>(file_slice: &'a str) -> &'a str {
    match WHITESPACE.find(file_slice) {
        Some(result) => return &file_slice[result.end()..],
        None => file_slice,
    }
}


fn xml_version<'a>(mut file_slice: &'a str) -> Option<&'a str> {
    let version: Regex = Regex::new(r#"\A<\?xml version="[0-9]\.[0-9]" encoding="UTF-8"\?>\n?"#).unwrap();
    file_slice = grab_whitespace(file_slice);
    match version.find(file_slice) {
        Some(result) => return Some(&file_slice[result.end()..]),
        None => return None,
    };
}

fn comment<'a>(mut file_slice: &'a str) -> Option<&'a str> {
    let comment: Regex = Regex::new(r"\A<!--(?:.|\n|\r\n)*-->?").unwrap();
    file_slice = grab_whitespace(file_slice);
    match comment.find(file_slice) {
        Some(result) => return Some(&file_slice[result.end()..]),
        None => return None,
    }
}