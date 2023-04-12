fn median(arr: &[i32]) -> f32 {
    let n = arr.len();
    if n % 2 == 0 {
        (arr[n/2] + arr[n/2-1]) as f32 / 2.0
    } else {
        arr[n/2] as f32
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    let median = median(&arr);
    println!("The median of {:?} is {}", arr, median);
}
