use crate::splitter::{split_word, SplitBy};
use crate::converter::{ captitalize_first_letter, convert_to_lower_case };

pub fn to_pascal_case(input: &str, split_by:SplitBy) -> String {
    let v:Vec<&str> = split_word(input, split_by);
    let mut temp: Vec<String> = Vec::new();
    for (_i, el) in v.iter().enumerate() {
        let mut a = convert_to_lower_case(el);
        a = captitalize_first_letter(&a);
        temp.push(a);
    }
    let string = temp.into_iter().collect::<String>();
    string
}

#[cfg(test)]
mod test {
    #[test]
    fn test_snake_case() {
        assert_eq!(super::to_pascal_case("hello_world", super::SplitBy::Underscore), "HelloWorld");
    }

    #[test]
    fn test_snake_case_repeated() {
        assert_eq!(super::to_pascal_case("hello_world_again", super::SplitBy::Underscore), "HelloWorldAgain");
    }

    #[test]
    fn test_kebab_case() {
        assert_eq!(super::to_pascal_case("hello-world", super::SplitBy::Hyphen), "HelloWorld");
    }

    #[test]
    fn test_kebab_case_repeated() {
        assert_eq!(super::to_pascal_case("hello-world-again", super::SplitBy::Hyphen), "HelloWorldAgain");
    }

    #[test]
    fn test_macro_case() {
        assert_eq!(super::to_pascal_case("HELLO_WORLD", super::SplitBy::Underscore), "HelloWorld");
    }

    #[test]
    fn test_macro_case_repeated() {
        assert_eq!(super::to_pascal_case("HELLO_WORLD_AGAIN", super::SplitBy::Underscore), "HelloWorldAgain");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(super::to_pascal_case("helloWorld", super::SplitBy::LowerToUpper), "HelloWorld");
    }

    #[test]
    fn test_camel_case_repeated() {
        assert_eq!(super::to_pascal_case("helloWorldAgain", super::SplitBy::LowerToUpper), "HelloWorldAgain");
    }

    #[test]
    fn test_train_case() {
        assert_eq!(super::to_pascal_case("Hello_World", super::SplitBy::Underscore), "HelloWorld");
    }

    #[test]
    fn test_train_case_repeated() {
        assert_eq!(super::to_pascal_case("Hello_World_Again", super::SplitBy::Underscore), "HelloWorldAgain");
    }

}
