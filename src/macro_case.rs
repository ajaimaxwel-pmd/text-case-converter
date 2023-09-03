use crate::splitter::{split_word, SplitBy};
use crate::converter::convert_to_upper_case;

pub fn to_macro_case(input: &str, split_by:SplitBy) -> String {
    let v:Vec<&str> = split_word(input, split_by);
    let mut temp: Vec<String> = Vec::new();
    for (_i, el) in v.iter().enumerate() {
        let a = convert_to_upper_case(el);
        temp.push(a);
    }
    let string = temp.join("_").to_string();
    string
}

#[cfg(test)]
mod test {
    #[test]
    fn test_snake_case() {
        assert_eq!(super::to_macro_case("hello_world", super::SplitBy::Underscore), "HELLO_WORLD");
    }

    #[test]
    fn test_snake_case_repeated() {
        assert_eq!(super::to_macro_case("hello_world_again", super::SplitBy::Underscore), "HELLO_WORLD_AGAIN");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(super::to_macro_case("helloWorld", super::SplitBy::LowerToUpper), "HELLO_WORLD");
    }

    #[test]
    fn test_camel_case_repeated() {
        assert_eq!(super::to_macro_case("helloWorldAgain", super::SplitBy::LowerToUpper), "HELLO_WORLD_AGAIN");
    }

    #[test]
    fn test_kebab_case() {
        assert_eq!(super::to_macro_case("hello-world", super::SplitBy::Hyphen), "HELLO_WORLD");
    }

    #[test]
    fn test_kebab_case_repeated() {
        assert_eq!(super::to_macro_case("hello-world-again", super::SplitBy::Hyphen), "HELLO_WORLD_AGAIN");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(super::to_macro_case("HelloWorld", super::SplitBy::UpperToLowerToUpper), "HELLO_WORLD");
    }

    #[test]
    fn test_pascal_case_repeated() {
        assert_eq!(super::to_macro_case("HelloWorldAgain", super::SplitBy::UpperToLowerToUpper), "HELLO_WORLD_AGAIN");
    }

    #[test]
    fn test_train_case() {
        assert_eq!(super::to_macro_case("Hello_World", super::SplitBy::Underscore), "HELLO_WORLD");
    }

    #[test]
    fn test_train_case_repeated() {
        assert_eq!(super::to_macro_case("Hello_World_Again", super::SplitBy::Underscore), "HELLO_WORLD_AGAIN");
    }

}
