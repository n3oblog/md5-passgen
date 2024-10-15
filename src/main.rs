use md5;
use std::io::{self, Write};
use clipboard::{ClipboardContext, ClipboardProvider};

fn generate_md5_hash(input: &str) -> String {
    let digest = md5::compute(input);
    let hash = format!("{:x}", digest);

    hash.chars().take(16).collect()
}

fn main() {
    print!("Enter string to hash: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read string");

    let input = input.trim();

    let hash = generate_md5_hash(input);
    println!("{}", hash);

    let mut clipboard: ClipboardContext = ClipboardProvider::new().expect("Cannot initialize clipboard");
    clipboard.set_contents(hash.clone()).expect("Cannot add md5 hash to clipboard");
    
}
