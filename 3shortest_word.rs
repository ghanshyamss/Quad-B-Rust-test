fn shortest_word(s: &str) -> Option<&str> {
    let mut shortest: Option<&str> = None;
    for word in s.split_whitespace() {
        shortest = match shortest {
            None => Some(word),
            Some(shortest_word) => {
                if word.len() < shortest_word.len() {
                    Some(word)
                } else {
                    Some(shortest_word)
                }
            }
        };
    }
    shortest
}

fn main() {
    let s = "aloo gobhi pudina panther hi hehe";
    match shortest_word(s) {
        Some(shortest) => println!("The shortest word is '{}'", shortest),
        None => println!("The string is empty"),
    }
}
