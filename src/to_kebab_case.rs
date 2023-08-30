pub fn to_kebab_case(word: &str) -> String {
    word.chars().map(|c| {
        if c.is_uppercase() {
            format!("-{}", c.to_lowercase().next().unwrap())
        } else {
            c.to_string()
        }
    }).collect()
}
