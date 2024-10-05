mod xml_parser;
mod model_conversion;

pub fn wrapper(text: &str) -> xml_parser::Node {
    return xml_parser::parse_xml(text);
    // include error hadnling in parsexml
}