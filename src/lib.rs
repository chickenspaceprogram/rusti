mod token_loader;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::token_loader;
        let file = include_str!(r"../tokens/8X.xml");
        let result = token_loader::wrapper(file);
    }
}