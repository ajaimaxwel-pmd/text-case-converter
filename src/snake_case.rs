use crate::splitter::{split_word, SplitBy};
use crate::converter::convert_to_lower_case;

pub fn to_snake_case(input: &str, split_by:SplitBy) -> String {
    let v:Vec<&str> = split_word(input, split_by);
    let mut temp: Vec<String> = Vec::new();
    for (_i, el) in v.iter().enumerate() {
        let a = convert_to_lower_case(el);
        temp.push(a);
    }
    let string = temp.join("_").to_string();
    string
}

#[cfg(test)]
mod test {
    #[test]
    fn test_kebab_case() {
        assert_eq!(super::to_snake_case("hello-world", super::SplitBy::Hyphen), "hello_world");
    }

    #[test]
    fn test_kebab_case_repeated() {
        assert_eq!(super::to_snake_case("hello-world-again", super::SplitBy::Hyphen), "hello_world_again");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(super::to_snake_case("helloWorld", super::SplitBy::LowerToUpper), "hello_world");
    }

    #[test]
    fn test_camel_case_repeated() {
        assert_eq!(super::to_snake_case("helloWorldAgain", super::SplitBy::LowerToUpper), "hello_world_again");
    }

    #[test]
    fn test_macro_case() {
        assert_eq!(super::to_snake_case("HELLO_WORLD", super::SplitBy::Underscore), "hello_world");
    }

    #[test]
    fn test_macro_case_repeated() {
        assert_eq!(super::to_snake_case("HELLO_WORLD_AGAIN", super::SplitBy::Underscore), "hello_world_again");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(super::to_snake_case("HelloWorld", super::SplitBy::UpperToLowerToUpper), "hello_world");
    }

    #[test]
    fn test_pascal_case_repeated() {
        assert_eq!(super::to_snake_case("HelloWorldAgain", super::SplitBy::UpperToLowerToUpper), "hello_world_again");
    }

    #[test]
    fn test_train_case() {
        assert_eq!(super::to_snake_case("Hello_World", super::SplitBy::Underscore), "hello_world");
    }

    #[test]
    fn test_train_case_repeated() {
        assert_eq!(super::to_snake_case("Hello_World_Again", super::SplitBy::Underscore), "hello_world_again");
    }

}
