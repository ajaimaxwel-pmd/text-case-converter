pub fn to_camel_case(word: &str) -> String {
    let mut camel_case_string = String::new();
    let mut capitalize_next = false;
    for (i, c) in word.chars().enumerate() {
        if c == '_' || c == '-' {
            capitalize_next = true;
            continue;
        }
        if i == 0 {
            camel_case_string.push(c.to_lowercase().next().unwrap());
        } else if capitalize_next {
            camel_case_string.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            camel_case_string.push(c);
        }
    }
    camel_case_string
}
