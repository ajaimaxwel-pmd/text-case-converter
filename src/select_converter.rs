#[path = "./to_pascal_case.rs"] mod to_pascal_case;
#[path = "./to_camel_case.rs"] mod to_camel_case;
#[path = "./to_kebab_case.rs"] mod to_kebab_case;
#[path = "./to_macro_case.rs"] mod to_macro_case;
#[path = "./to_train_case.rs"] mod to_train_case;
#[path = "./to_snake_case.rs"] mod to_snake_case;

use to_pascal_case::to_pascal_case;
use to_camel_case::to_camel_case;
use to_kebab_case::to_kebab_case;
use to_macro_case::to_macro_case;
use to_train_case::to_train_case;
use to_snake_case::to_snake_case;

use crate::case_types::CaseType;

pub fn select_converter(case_type: CaseType) -> fn(&str) -> String {
    return match case_type {
        CaseType::Pascal => to_pascal_case,
        CaseType::Camel => to_camel_case,
        CaseType::Snake => to_snake_case,
        CaseType::Kebab => to_kebab_case,
        CaseType::Macro => to_macro_case,
        CaseType::Train => to_train_case,
        _ => panic!("Invalid to case type"),
    };
}
