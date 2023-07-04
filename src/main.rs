use std::env;

fn main() {
    // Read the command-line argument for the message
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a message.");
        return;
    }

    // Skip the first argument, which is the program's name
    let mut result = String::new();
    for arg in args.iter().skip(1) {
        result.push_str(arg.replace('\n', " ").as_str());
        result.push(' ');
    }

    result = result.trim().to_string();

    // Count words and characters
    let word_count = result.split_whitespace().count();
    let char_count = result.replace(" ", "").chars().count();
    let char_count_withspace = result.chars().count();

    // Print the results
    println!("Echo: {result}");
    println!("Word count: {word_count}");
    println!("Character count: {char_count}");
    println!("Character count with space: {char_count_withspace}");
}
