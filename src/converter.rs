use crate::splitter::SplitBy;
use crate::case_types::CaseType;
use crate::camel_case::to_camel_case;
use crate::pascal_case::to_pascal_case;
use crate::kebab_case::to_kebab_case;
use crate::snake_case::to_snake_case;
use crate::macro_case::to_macro_case;
use crate::train_case::to_train_case;

pub fn select_converter(case_type: &CaseType) -> fn(&str, SplitBy) -> String {
    return match case_type {
        CaseType::Pascal => to_pascal_case,
        CaseType::Camel => to_camel_case,
        CaseType::Snake => to_snake_case,
        CaseType::Kebab => to_kebab_case,
        CaseType::Macro => to_macro_case,
        CaseType::Train => to_train_case,
    };
}

pub fn convert_to_lower_case(input: &str) -> String {
    input.to_lowercase().to_string()
}

pub fn convert_to_upper_case(input: &str) -> String {
    input.to_uppercase().to_string()
}

pub fn captitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
