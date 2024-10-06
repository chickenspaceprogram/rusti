use std::collections::HashMap;
use crate::token_loader::{xml_parser, model_conversion};

pub struct Token {
    lsb: u8,
    msb: Option<u8>,
    model: model_conversion::Model,
    os: String,
}

pub fn parse_dom(main_node: xml_parser::Node) -> HashMap<String, Token> {
    return HashMap::new();
}