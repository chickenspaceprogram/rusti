use std::collections::HashMap;
use crate::token_loader::{xml_parser, model_conversion};

#[derive(Debug)]
pub struct TokenInfo {
    lsb: u8,
    msb: Option<u8>,
    since: Calc,
    until: Option<Calc>, // if this is None, then the token is valid for all calcs after since
    os: String,
}

#[derive(Debug)]
pub struct Token {
    info: Option<TokenInfo>,
    longest_token
}

#[derive(Debug)]
pub struct Calc {
    model: model_conversion::Model,
    version: String,
}

pub fn parse_dom(main_node: xml_parser::Node) -> HashMap<String, Token> {
    return HashMap::new();
}