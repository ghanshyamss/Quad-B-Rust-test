fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let s = "Meow,mooo!";
    let reversed = reverse_string(s);
    println!("{} => {}", s, reversed);
}
