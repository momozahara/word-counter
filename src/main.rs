use std::env;

fn normal_run(args: Vec<String>) {
    let mut result = String::new();
    for arg in args.iter().skip(1) {
        result.push_str(arg.replace("\\n", "\n").replace("\n", " ").as_str());
        result.push(' ');
    }

    result = result.trim().to_string();

    let word_count = result.split_whitespace().count();
    let char_count = result.replace(" ", "").chars().count();
    let char_count_withspace = result.chars().count();

    println!("Echo: {result}");
    println!("Word count: {word_count}");
    println!("Character count: {char_count}");
    println!("Character count with space: {char_count_withspace}");
}

fn line_run(args: Vec<String>) {
    let mut result = String::new();
    for arg in args.iter().skip(1) {
        result.push_str(arg.replace("\\n", "\n").as_str());
    }

    let line_count = result.lines().count();

    println!("Line count: {line_count}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a message.");
        return;
    }

    let line_flag = args.iter().find(|a| a.as_str() == "-l");

    match line_flag {
        Some(_) => line_run(args),
        None => normal_run(args),
    }
}
