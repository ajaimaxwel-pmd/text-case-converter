#![allow(unused)]
mod to_pascal_case;
mod to_camel_case;
mod to_kebab_case;
mod to_macro_case;
mod to_train_case;
mod to_snake_case;

extern crate regex;

use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use regex::Regex;
use to_pascal_case::to_pascal_case;
use to_camel_case::to_camel_case;
use to_kebab_case::to_kebab_case;
use to_macro_case::to_macro_case;
use to_train_case::to_train_case;
use to_snake_case::to_snake_case;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() -> io::Result<()> {
    let matches = Command::new("TextConverter")
        .version("1.0.0")
        .author("Your Name <youremail@example.com>")
        .about("Converts text in a file to different cases")
        .arg(Arg::new("from_case_type").short('f').long("from").help("Sets the type of case to convert from").required(true))
        .arg(Arg::new("to_case_type").short('t').long("to").help("Sets the type of case to convert to").required(true))
        .arg(Arg::new("file").short('p').long("path").help("Sets the input file to use").required(true))
        .get_matches();

    let from_case_type = matches.get_one::<String>("from_case_type").unwrap();
    println!("-----from_case_type {}", from_case_type);
    print_type_of(&from_case_type); 
    let to_case_type = matches.get_one::<String>("to_case_type").unwrap();
    let file_path = matches.get_one::<String>("file").unwrap();

    let input_path = Path::new(file_path);
    let input_file = File::open(&input_path)?;
    let reader = io::BufReader::new(input_file);

    let output_path = Path::new("output.txt");
    let mut output_file = File::create(&output_path)?;

    let re = match (from_case_type).as_str() {
        "pascal" => Regex::new(r"\b([A-Z][a-z0-9]*)+\b").unwrap(),
        "camel" => Regex::new(r"\b[a-z][a-zA-Z]*[A-Z][a-zA-Z]*\b").unwrap(),
        "snake" => Regex::new(r"\b[a-z0-9]+(_[a-z0-9]+)*\b").unwrap(),
        "kebab" => Regex::new(r"\b[a-z0-9]+(-[a-z0-9]+)*\b").unwrap(),
        "macro" => Regex::new(r"\b[A-Z0-9]+(_[A-Z0-9]+)*\b").unwrap(),
        "train" => Regex::new(r"\b([A-Z][a-z0-9]*-)*[A-Z][a-z0-9]*\b").unwrap(),
        _ => panic!("Invalid from case type"),
    };

    for line in reader.lines() {
        let line = line?;
        let mut replacements: Vec<(String, String)> = Vec::new();
        
        for cap in re.captures_iter(&line) {
            let from_case_word = &cap[0];
            let to_case_word = match (to_case_type).as_str() {
                "pascal" => to_pascal_case(&from_case_word),
                "camel" => to_camel_case(&from_case_word),
                "snake" => to_snake_case(&from_case_word),
                "kebab" => to_kebab_case(&from_case_word),
                "macro" => to_macro_case(&from_case_word),
                "train" => to_train_case(&from_case_word),
                _ => panic!("Invalid to case type"),
            };
            replacements.push((from_case_word.to_string(), to_case_word));
        }

        let mut new_line = line.clone();
        for (old, new) in replacements {
            new_line = new_line.replace(&old, &new);
        }

        writeln!(output_file, "{}", new_line)?;
    }
    Ok(())
}
