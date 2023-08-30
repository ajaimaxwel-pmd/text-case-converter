pub fn to_snake_case(word: &str) -> String {
    word.chars().map(|c| {
        if c.is_uppercase() {
            format!("_{}", c.to_lowercase().next().unwrap())
        } else {
            c.to_string()
        }
    }).collect()
}
