pub fn to_train_case(word: &str) -> String {
    let mut train_case_string = String::new();
    let mut capitalize_next = true;
    for c in word.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
            train_case_string.push('-');
            continue;
        }
        if capitalize_next {
            train_case_string.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            train_case_string.push(c.to_lowercase().next().unwrap());
        }
    }
    train_case_string
}
