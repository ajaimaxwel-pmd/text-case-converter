use crate::splitter::{split_word, SplitBy};
use crate::converter::{convert_to_lower_case, captitalize_first_letter};

pub fn to_train_case(input: &str, split_by:SplitBy) -> String {
    let v:Vec<&str> = split_word(input, split_by);
    let mut temp: Vec<String> = Vec::new();
    for (_i, el) in v.iter().enumerate() {
        let mut a = convert_to_lower_case(el);
        a = captitalize_first_letter(&a);
        temp.push(a);
    }
    let string = temp.join("-").to_string();
    string
}

#[cfg(test)]
mod test {
    #[test]
    fn test_kebab_case() {
        assert_eq!(super::to_train_case("hello-world", super::SplitBy::Hyphen), "Hello-World");
    }

    #[test]
    fn test_kebab_case_repeated() {
        assert_eq!(super::to_train_case("hello-world-again", super::SplitBy::Hyphen), "Hello-World-Again");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(super::to_train_case("helloWorld", super::SplitBy::LowerToUpper), "Hello-World");
    }

    #[test]
    fn test_camel_case_repeated() {
        assert_eq!(super::to_train_case("helloWorldAgain", super::SplitBy::LowerToUpper), "Hello-World-Again");
    }

    #[test]
    fn test_macro_case() {
        assert_eq!(super::to_train_case("Hello-World", super::SplitBy::Underscore), "Hello-World");
    }

    #[test]
    fn test_macro_case_repeated() {
        assert_eq!(super::to_train_case("Hello-World-Again", super::SplitBy::Underscore), "Hello-World-Again");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(super::to_train_case("HelloWorld", super::SplitBy::UpperToLowerToUpper), "Hello-World");
    }

    #[test]
    fn test_pascal_case_repeated() {
        assert_eq!(super::to_train_case("HelloWorldAgain", super::SplitBy::UpperToLowerToUpper), "Hello-World-Again");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(super::to_train_case("hello_world", super::SplitBy::Underscore), "Hello-World");
    }

    #[test]
    fn test_snake_case_repeated() {
        assert_eq!(super::to_train_case("hello_world_again", super::SplitBy::Underscore), "Hello-World-Again");
    }

}
