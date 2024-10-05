mod token_loader;

fn main() {
    let file = include_str!(r"../tokens/8X.xml");
    let result = token_loader::wrapper(file);
}