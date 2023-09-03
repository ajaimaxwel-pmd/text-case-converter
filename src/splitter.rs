use crate::case_types::CaseType;

pub enum SplitBy {
    Underscore,
    Hyphen,
    LowerToUpper,
    UpperToLowerToUpper,
}

pub fn get_split_type(case_type: &CaseType) -> SplitBy {
    return match case_type {
        CaseType::Pascal => SplitBy::UpperToLowerToUpper,
        CaseType::Snake | CaseType::Macro | CaseType::Train => SplitBy::Underscore,
        CaseType::Kebab => SplitBy::Hyphen,
        CaseType::Camel => SplitBy::LowerToUpper
    }
}

pub fn split_word(word: &str, split_type: SplitBy) -> Vec<&str> {
    let result: Vec<&str> = match split_type {
        SplitBy::UpperToLowerToUpper => split_word_by_upper_to_lower_to_upper_case(word), //Pascal
        SplitBy::LowerToUpper => split_word_by_lower_to_upper_case(word), // Camel
        SplitBy::Hyphen => split_word_by_symbol(word),
        SplitBy::Underscore => split_word_by_symbol(word),
    };
    result
}

pub fn split_word_by_symbol(word: &str) -> Vec<&str> {
    let result: Vec<&str> = word.split(|c: char| c == '-' || c == '_')
        .filter(|&s| !s.is_empty())
        .collect();
    result
}

pub fn split_word_by_upper_to_lower_to_upper_case(word: &str) -> Vec<&str> {
    let re = regex::Regex::new(r"([A-Z][a-z]+)").expect("Unable to create regex pattern");
    let mut result: Vec<&str> = Vec::new();  // Initialize the vector
    for field in re.find_iter(word) {
        result.push(field.as_str());  // push each field into the result vector
    }
    result  // Return result
}

pub fn split_word_by_lower_to_upper_case(word: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut last = 0;
    let chars: Vec<char> = word.chars().collect();
    for (i, &item) in chars.iter().enumerate().skip(1) {
        if item.is_uppercase() && chars[i - 1].is_lowercase() {
            result.push(&word[last..i]);
            last = i;
        }
    }
    result.push(&word[last..]);
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn test_split_camel_case() {
        let expected = ["hello" , "World"];
        assert_eq!(super::split_word_by_lower_to_upper_case("helloWorld"), expected);
    }

    #[test]
    fn test_split_camel_case_repeated() {
        let expected = ["hello" , "World", "Again"];
        assert_eq!(super::split_word_by_lower_to_upper_case("helloWorldAgain"), expected);
    }
}