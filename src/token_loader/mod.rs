use regex::Regex;
use std::collections::HashMap;
/// A struct representing data from a token.
/// This is intended to be passed around between functions and more broadly to store the state of this recursive descent parser.
/// It contains the bytes of the token (if the token only has one byte, byte1 is set to None)
/// slice_start is a way to track what part of the file text the parser is currently focused on.
struct TokenInfo<'a> {
    byte1: Option<u8>,
    byte2: Option<u8>,
    model: Model,
    version:
    slice: &'a str,
}

enum Model {
    TI82,
    TI83,
    TI83P,
    TI84P,
    TI84PCE,
    TI84PCSE,
}

// returns a string containing the start of what's after this string.
fn version(file_slice: &str) -> Option<usize> {
    let version: Regex = Regex::new("<\\?xml version=\"[0-9]\\.[0-9]\" encoding=\"UTF-8\"\\?>\\n?").unwrap();
    match version.find(file_slice) {
        Some(result) => return Some(result.end()),
        None => return None,
    };
}

fn comment(file_slice: &str) -> Option<usize> {
    let comment: Regex = Regex::new(r"<!--[\r]?(?:.|\n)*-->\n?").unwrap();
    match comment.find(file_slice) {
        Some(result) => return Some(result.end()),
        None => return None,
    }
}

fn tokens(file_slice: &str) -> TokenInfo {
    let two_byte_tag: Regex = Regex::new("<two-byte value=\"\\$[0-9a-fA-F]{2}\">\n?").expect("The two-byte token regex has not compiled.");
    match two_byte_tag.find(file_slice) {
        Some(result) => {
            let mut token_info: TokenInfo = {
                slice = &file_slice[result.end()..],
                byte1 = Some(result.as_str()[(result.start() + 19)..(result.start() + 21)].parse().unwrap()), // this is cursed, do this with capturing grps instead wtf
                byte2 = None,
            }
        }
        None => {
            token_info.slice = &file_slice;
            token_info.byte1 = None;
            token_info.byte2 = None;
            return token_info; // no idea if this is how you're supposed to do this in Rust
            
        }
    }
}

fn token(file_slice: TokenInfo) {

}

fn two_byte(file_slice: &str) {

}

#[test]
fn tests() {
    use std::fs;

    let contents = fs::read_to_string("tokens/8X.xml").expect("File did not open.");
    assert!(version(&contents).unwrap() == 39);
}