mod xml_parser;
mod model_conversion;
mod dom_parser;


pub fn wrapper(text: &str) -> Result<xml_parser::Node, xml_parser::XMLParsingError> {
    return xml_parser::parse_xml(text);
    // include error hadnling in parsexml
}