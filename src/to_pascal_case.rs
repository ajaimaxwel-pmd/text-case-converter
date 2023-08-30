pub fn to_pascal_case(word: &str) -> String {
    let mut pascal_case_string = String::new();
    let mut chars = word.chars();
    // Uppercase the first character.
    if let Some(first_char) = chars.next() {
        pascal_case_string.push(first_char.to_uppercase().next().unwrap());
    }
    // Copy the rest as-is.
    pascal_case_string.extend(chars);
    pascal_case_string
}
