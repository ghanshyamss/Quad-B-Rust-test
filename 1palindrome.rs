use std::io;

fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    s.to_lowercase() == s.to_lowercase().chars().rev().collect::<String>()
}

fn main() {
    println!("Enter a string to check if it's a palindrome:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();
    println!("Is '{}' a palindrome? {}", input, is_palindrome(input));
}
