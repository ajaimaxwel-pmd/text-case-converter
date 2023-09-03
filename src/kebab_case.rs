use crate::splitter::{split_word, SplitBy};
use crate::converter::convert_to_lower_case;

pub fn to_kebab_case(input: &str, split_by:SplitBy) -> String {
    let v:Vec<&str> = split_word(input, split_by);
    let mut temp: Vec<String> = Vec::new();
    for (_i, el) in v.iter().enumerate() {
        let a = convert_to_lower_case(el);
        temp.push(a);
    }
    let string = temp.join("-").to_string();
    string
}

#[cfg(test)]
mod test {
    #[test]
    fn test_snake_case() {
        assert_eq!(super::to_kebab_case("hello_world", super::SplitBy::Underscore), "hello-world");
    }

    #[test]
    fn test_snake_case_repeated() {
        assert_eq!(super::to_kebab_case("hello_world_again", super::SplitBy::Underscore), "hello-world-again");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(super::to_kebab_case("helloWorld", super::SplitBy::LowerToUpper), "hello-world");
    }

    #[test]
    fn test_camel_case_repeated() {
        assert_eq!(super::to_kebab_case("helloWorldAgain", super::SplitBy::LowerToUpper), "hello-world-again");
    }

    #[test]
    fn test_macro_case() {
        assert_eq!(super::to_kebab_case("HELLO_WORLD", super::SplitBy::Underscore), "hello-world");
    }

    #[test]
    fn test_macro_case_repeated() {
        assert_eq!(super::to_kebab_case("HELLO_WORLD_AGAIN", super::SplitBy::Underscore), "hello-world-again");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(super::to_kebab_case("HelloWorld", super::SplitBy::UpperToLowerToUpper), "hello-world");
    }

    #[test]
    fn test_pascal_case_repeated() {
        assert_eq!(super::to_kebab_case("HelloWorldAgain", super::SplitBy::UpperToLowerToUpper), "hello-world-again");
    }

    #[test]
    fn test_train_case() {
        assert_eq!(super::to_kebab_case("Hello_World", super::SplitBy::Underscore), "hello-world");
    }

    #[test]
    fn test_train_case_repeated() {
        assert_eq!(super::to_kebab_case("Hello_World_Again", super::SplitBy::Underscore), "hello-world-again");
    }

}
