fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = &strs[0][..];
    for s in &strs[1..] {
        while !s.starts_with(prefix) {
            prefix = &prefix[..prefix.len()-1];
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix.to_string()
}

fn main() {
    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let prefix = longest_common_prefix(&strs);
    println!("The longest common prefix of {:?} is '{}'", strs, prefix);
}
