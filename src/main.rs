use std::env;

fn main() {
    // Read the command-line argument for the message
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a message.");
        return;
    }
    let contents = &args[1];

    // Count words and characters
    let word_count = contents.split_whitespace().count();
    let char_count = contents.chars().count();
    let char_nwhitespace_count = contents.replace(" ", "").chars().count();

    // Print the results
    println!("Echo: {contents}");
    println!("Word count: {word_count}");
    println!("Character count: {char_count}");
    println!("Character no space count: {char_nwhitespace_count}");
}
