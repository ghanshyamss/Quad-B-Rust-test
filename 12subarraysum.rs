fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = 0;
    let mut max_ending_here = 0;
    for &x in arr {
        max_ending_here = i32::max(x, max_ending_here + x);
        max_so_far = i32::max(max_so_far, max_ending_here);
    }
    return max_so_far;
}

fn main() {
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
