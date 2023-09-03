extern crate regex;
use regex::Regex;

use crate::case_types::CaseType;

pub fn case_checkers(case_type: &CaseType) -> Regex {
    return match case_type {
        CaseType::Pascal => Regex::new(r"\b([A-Z][a-z0-9]*)+\b").unwrap(),
        CaseType::Camel => Regex::new(r"\b[a-z][a-zA-Z]*[A-Z][a-zA-Z]*\b").unwrap(),
        CaseType::Snake => Regex::new(r"\b[a-z0-9]+(_[a-z0-9]+)*\b").unwrap(),
        CaseType::Kebab => Regex::new(r"\b[a-z0-9]+(-[a-z0-9]+)*\b").unwrap(),
        CaseType::Macro => Regex::new(r"\b[A-Z0-9]+(_[A-Z0-9]+)*\b").unwrap(),
        CaseType::Train => Regex::new(r"\b([A-Z][a-z0-9]*-)*[A-Z][a-z0-9]*\b").unwrap(),
    };
}
