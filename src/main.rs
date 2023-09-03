mod checker;
mod splitter;
mod case_types;
mod converter;

mod camel_case;
mod kebab_case;
mod macro_case;
mod pascal_case;
mod snake_case;
mod train_case;

use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use checker::case_checkers;
use case_types::CaseType;
use converter::select_converter;
use splitter::{get_split_type, SplitBy};

fn main() -> io::Result<()> {
    let matches = Command::new("TextConverter")
        .version("1.0.0")
        .author("Your Name <youremail@example.com>")
        .about("Converts text in a file to different cases")
        .arg(Arg::new("from_case_type").short('f').long("from").help("Sets the type of case to convert from").required(true))
        .arg(Arg::new("to_case_type").short('t').long("to").help("Sets the type of case to convert to").required(true))
        .arg(Arg::new("file").short('p').long("path").help("Sets the input file to use").required(true))
        .get_matches();

    let from_case_type_arg = matches.get_one::<String>("from_case_type").unwrap();
    let to_case_type_arg = matches.get_one::<String>("to_case_type").unwrap();
    let file_path = matches.get_one::<String>("file").unwrap();

    let from_case_type: CaseType = from_case_type_arg.to_owned().parse().unwrap();
    let to_case_type: CaseType = to_case_type_arg.to_owned().parse().unwrap();

    let input_path = Path::new(file_path);
    let input_file = File::open(&input_path)?;
    let reader = io::BufReader::new(input_file);

    let output_path = Path::new("output.txt");
    let mut output_file = File::create(&output_path)?;

    let re = case_checkers(&from_case_type);
    let case_converter: fn(&str, SplitBy) -> String = select_converter(&to_case_type);

    for line in reader.lines() {
        let line = line?;
        let mut replacements: Vec<(String, String)> = Vec::new();
        
        for cap in re.captures_iter(&line) {
            let split_by: SplitBy = get_split_type(&from_case_type);

            let from_case_word = &cap[0];
            let to_case_word = case_converter(&from_case_word, split_by);
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
