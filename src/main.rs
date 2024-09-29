use std::{fs, vec};
use regex::Regex;
use std::collections::HashMap;
fn main() {
    let text = fs::read_to_string("tokens/8X.xml").expect("The fie did not open.");
    let os_model_regex: Regex = Regex::new(r"<model>(TI-8[0-9]\+?(?:CE|CSE)?)</model>\s*<os-version>([0-9.]+)</os-version>").unwrap(); // regexes are always cursed
    let result = os_model_regex.captures_iter(&text).into_iter();
    let mut results_hash_map: HashMap<String, vec> = HashMap::new()
    for i in result {
        i.
    }
}