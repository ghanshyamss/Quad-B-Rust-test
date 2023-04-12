fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    s.to_lowercase() == s.to_lowercase().chars().rev().collect::<String>()
}

fn main() {
    let test_string = "A man, a plan, a canal, Panama!";
    println!("Is '{}' a palindrome? {}", test_string, is_palindrome(test_string));
}
