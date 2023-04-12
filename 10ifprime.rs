fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f32).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let n = 17;
    if is_prime(n) {
        println!("{} is prime", n);
    } else {
        println!("{} is not prime", n);
    }
}
