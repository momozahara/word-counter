use regex::Regex;
use std::{collections::HashSet, env};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Flags {
    Line,
    Echo,
}

fn remove_extra_whitespace(input: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(input, " ").to_string()
}

fn normal_run(args: Vec<String>, flags: HashSet<Flags>) {
    let mut result = String::new();
    for arg in args.iter().skip(1) {
        result.push_str(
            remove_extra_whitespace(arg.replace("\\n", "\n").replace("\n", " ").as_str()).as_str(),
        );
        result.push(' ');
    }

    result = result.trim().to_string();

    let word_count = result.split_whitespace().count();
    let char_count = result.replace(" ", "").chars().count();

    match flags.contains(&Flags::Echo) {
        false => {
            println!("Echo: {result}");
            println!("Word count: {word_count}");
            println!("Character count: {char_count}");
        }
        true => {
            println!("{word_count}");
            println!("{char_count}");
        }
    }
}

fn line_run(args: Vec<String>, flags: HashSet<Flags>) {
    let mut result = String::new();
    for arg in args.iter().skip(1) {
        result.push_str(arg.replace("\\n", "\n").as_str());
    }

    let line_count = result.lines().count();

    match flags.contains(&Flags::Echo) {
        false => {
            println!("Line count: {line_count}");
        }
        true => {
            println!("{line_count}");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a message.");
        return;
    }

    let flags: HashSet<Flags> = args
        .iter()
        .filter_map(|a| match a.as_str() {
            "-e" => Some(Flags::Echo),
            "-l" => Some(Flags::Line),
            _ => None,
        })
        .collect();

    match flags.contains(&Flags::Line) {
        true => line_run(args, flags),
        false => normal_run(args, flags),
    }
}
