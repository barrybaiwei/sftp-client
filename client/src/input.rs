use std::path::PathBuf;
use std::io::{self, Write};

pub fn string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    String::from(input_text.trim())
}

pub fn prompt_path(prompt: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(string(prompt));
    path
}

pub fn path() -> PathBuf {
    prompt_path("\nEnter path: ")
}

pub fn positive(prompt: &str) -> usize {
    string(prompt).parse::<usize>().unwrap()
}
